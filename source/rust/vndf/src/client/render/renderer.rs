use std::fmt::Write;

use nalgebra::{
    cast,
    Mat4,
    Ortho3,
    Iso3,
    Vec2,
    Vec3,
    ToHomogeneous,
    Norm,
};

use client::interface::Frame;
use client::window::Window;
use client::render::base::{
    color,
    Graphics,
};
use client::render::draw::{
    GlyphDrawer,
    ShipDrawer,
};
use client::render::camera::{Camera};
use shared::util::angle_of;

const SHIP_SIZE: f32 = 30.0;

pub struct Renderer {
    glyph_drawer  : GlyphDrawer,
    triangle      : ShipDrawer,
    line          : ShipDrawer,
    pub camera    : Camera,
    line_height   : f32,
    scaling_factor: f32,
}

impl Renderer {
    pub fn new(window: &Window, scaling_factor: f32) -> Renderer {
        let mut graphics = window.create_graphics();

        let font_height = 18.0 * scaling_factor;
        
        let glyph_drawer = GlyphDrawer::new(&mut graphics, font_height as u32);
        let triangle     = ShipDrawer::triangle(&mut graphics);
        let line         = ShipDrawer::line(&mut graphics);

        Renderer {
            glyph_drawer  : glyph_drawer,
            triangle      : triangle,
            line          : line,
            camera        : Camera::new(),
            line_height   : font_height,
            scaling_factor: scaling_factor,
        }
    }

    /// get new ortho transform matrix based on window size specified
    fn get_transform(size: (u32,u32)) -> Mat4<f64> {
        Ortho3::new(
            size.0 as f64, size.1 as f64,
            -1.0, 1.0
                ).to_mat()
    }

    /// transforms camera z-positioning
    /// specify zoom-out level
    fn get_transform_camera(size: (u32,u32), z: f64) -> Mat4<f64> {
        let mut z = z.abs();
        if z < 1.0 { z = 1.0; }
        let mat = Ortho3::new(size.0 as f64 * z, size.1 as f64 * z,
                              -1.0,1.0
                              ).to_mat();
        mat
    }
    
    /// translates transform, used for camera positioning
    fn translate(transform: Mat4<f64>, pos: Vec2<f64>) -> Mat4<f64> {
        let translation = Iso3::new(
            Vec3::new(pos[0], pos[1], 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            );

        transform * translation.to_homogeneous()
    }

    pub fn render(
        &mut self,
        output : &[String],
        command: (&str,usize),
        frame  : &Frame,
        window : &Window,
        ) {
        let     window_size = window.get_size();
        let mut graphics    = window.create_graphics();

        graphics.clear();

        let transform  = Renderer::get_transform(window_size);
        let cam_pos = self.camera.update(&frame,None);
        let cam_trans = Renderer::get_transform_camera(window_size, self.camera.zoom);
        let world_trans = Renderer::translate(cam_trans,cam_pos);
        let screen_trans = Renderer::translate(transform,cam_pos);
        
        let transform: Mat4<f32> = cast(transform);
        let world_trans: Mat4<f32> = cast(world_trans);
        let screen_trans: Mat4<f32> = cast(screen_trans);

        let vec2_scaled = Vec2::new(1.0,1.0) *
            self.scaling_factor *
            (self.camera.zoom as f32);
        let vec2_text_scaled = Vec2::new(1.0,1.0) *
            (self.camera.zoom as f32);
        
        self.render_console(output, command, window_size, transform, &mut graphics);

        // draw ship selection, where necessary
        for id in frame.select_ids.iter() {
            if let Some(ship) = frame.ships.get(&id) {
                let position = ship.position + Vec2::new(0.0, 2.0 * self.camera.zoom);

                let translation = Iso3::new(
                    Vec3::new(position.x as f32, position.y as f32, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                );
                let transform = world_trans * translation.to_homogeneous();

                self.triangle.draw(
                    vec2_scaled.x * SHIP_SIZE * 1.25,
                    color::Colors::white(),
                    transform,
                    &mut graphics,
                );
            }
        }

        for (ship_id, ship) in &frame.ships {
            let pos_offset    = Vec2::new(SHIP_SIZE, 10.0);
            let line_advance  = Vec2::new(0.0, -self.line_height);

            let ship_position: Vec2<f32> = cast(ship.position);
            let ship_velocity: Vec2<f32> = cast(ship.velocity);

            // draw ship velocity line
            let transform = Iso3::new(
                Vec3::new(ship.position.x as f32, ship.position.y as f32, 0.0),
                Vec3::new(
                    0.0,
                    0.0,
                    angle_of(ship_velocity),
                ),
            );
            self.line.draw(
                vec2_scaled.x * ship_velocity.norm() * 50.0,
                color::Colors::red(),
                world_trans * transform.to_homogeneous(),
                &mut graphics,
            );

            let mut color = color::Colors::blue();
            if let Some(sid) = frame.ship_id {
                if *ship_id == sid  { color = color::Colors::green_spring(); }
            }

            let translation = Iso3::new(
                Vec3::new(ship_position.x, ship_position.y, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            );
            let transform = world_trans * translation.to_homogeneous();

            self.triangle.draw(
                vec2_scaled.x * SHIP_SIZE,
                color,
                transform,
                &mut graphics,
            );

            // draw ship id
            self.glyph_drawer.draw(
                &ship_id.to_string(),
                ship_position - line_advance + Vec2::new(0.0,5.0),
                vec2_text_scaled,
                self.camera.zoom,
                color::Colors::white(),
                true,
                screen_trans,
                &mut graphics,
                );

            // draw ship broadcast
            if let Some(ship_comm) = frame.broadcasts.get(&ship_id) {
                self.glyph_drawer.draw(
                    ship_comm,
                    ship_position + line_advance - Vec2::new(0.0, SHIP_SIZE),
                    vec2_text_scaled,
                    self.camera.zoom,
                    color::Colors::white(),
                    true,
                    screen_trans,
                    &mut graphics,
                    );
            }

            // draw ship position
            let pos = format!("pos: ({:.2}, {:.2})", ship.position[0], ship.position[1]);
            self.glyph_drawer.draw(
                &pos,
                ship_position + pos_offset,
                vec2_text_scaled,
                self.camera.zoom,
                color::Colors::white(),
                false,
                screen_trans,
                &mut graphics,
                );

            // draw ship velocity
            let vel = format!("vel: ({:.2}, {:.2})", ship.velocity[0], ship.velocity[1]);
            self.glyph_drawer.draw(
                &vel,
                ship_position + pos_offset + line_advance,
                vec2_text_scaled,
                self.camera.zoom,
                color::Colors::white(),
                false,
                screen_trans,
                &mut graphics,
                );
        }
        
        graphics.flush();
    }

    fn render_console(&mut self, output : &[String], command: (&str, usize), window_size: (u32, u32), transform: Mat4<f32>, graphics: &mut Graphics) {
        let advance_x   = self.glyph_drawer.advance_x;
        let line_height = self.line_height;

        for (y, line) in output.iter().enumerate() {
            self.glyph_drawer.draw(
                &line,
                position_cli(0, y, advance_x, line_height, window_size),
                Vec2::new(1.0,1.0),
                1.0,
                color::Colors::white(),
                false,
                transform,
                graphics,
            );
        }

        let mut command_line = String::new();
        let prompt_ypos = 23;

        write!(&mut command_line, "> {}", command.0)
            .unwrap_or_else(|e| panic!("Error writing to String: {}", e));


        self.glyph_drawer.draw(
            &command_line,
            position_cli(0, prompt_ypos, advance_x, line_height, window_size),
            Vec2::new(1.0,1.0),
            1.0,
            color::Colors::white(),
            false,
            transform,
            graphics,
        );

         //draw cursor position in prompt
        self.glyph_drawer.draw(
            &"_".to_string(),
            position_cli(command.1 + 2, prompt_ypos, advance_x, line_height, window_size),
            Vec2::new(1.0,1.0),
            1.0,
            color::Colors::white(),
            false,
            transform,
            graphics,
        );
    }
}


/// This is used to position CLI text
/// It takes in to account the window sizing
fn position_cli(
    x          : usize,
    y          : usize,
    advance_x  : f32,
    line_height: f32,
    window_size: (u32, u32),
) -> Vec2<f32> {
    let (width, height) = window_size;

    let pad_x = 10.0;
    let pad_y = 30.0;

    Vec2::new(
        (-1.0 * ((width as f32 / 2.0) - pad_x)) + advance_x * x as f32,
        ((height as f32 / 2.0) - pad_y) + line_height * (y as f32 * -1.0),
    )
}