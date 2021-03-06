use std::marker::PhantomData;
use std::f64::consts::PI;

use gfx;
use gfx_device_gl as gl;
use nalgebra::Mat4;

use client::graphics::base::{
    Batch,
    Graphics,
};
use shared::color;


static VERTEX_SRC: &'static [u8] = b"
        #version 120

        attribute vec2 pos;

        uniform mat4  transform;
        uniform float size;
        uniform vec3  color;

        varying vec3 v_color;

        void main() {
                gl_Position = transform * vec4(pos * size, 0.0, 1.0);
                v_color = color;
        }
";

static FRAGMENT_SRC: &'static [u8] = b"
        #version 120

        varying vec3 v_color;

        void main() {
                gl_FragColor = vec4(v_color, 1.0);
        }
";


gfx_vertex!(Vertex {
    pos@ pos: [f32; 2],
});


gfx_parameters!(Params {
    transform@ transform: [[f32; 4]; 4],
    size     @ size     : f32,
    color        @ color    : [f32; 3],
});


pub struct ShapeDrawer {
    batch: Batch<Params<gl::Resources>>,
}

impl ShapeDrawer {
    pub fn new(
        graphics : &mut Graphics,
        primitive: gfx::PrimitiveType,
        vertices : &[Vertex]
            ) -> ShapeDrawer {
        let batch = Batch::new(
            graphics,
            VERTEX_SRC, FRAGMENT_SRC,
            primitive,
            vertices,
            );

        ShapeDrawer {
            batch: batch,
        }
    }

    pub fn ship(graphics: &mut Graphics) -> ShapeDrawer {
        ShapeDrawer::new(
            graphics,
            gfx::PrimitiveType::TriangleStrip,
            &[
                Vertex { pos: [ -0.5, -0.5 ] },
                Vertex { pos: [  0.5, -0.5 ] },
                Vertex { pos: [  0.0,  0.5 ] },
                ],
            )
    }

    pub fn ship_group(graphics: &mut Graphics) -> ShapeDrawer {
        ShapeDrawer::new(
            graphics,
            gfx::PrimitiveType::TriangleList,
            &[
                // slightly taller center tri
                Vertex { pos: [ 0., 0.6 ] },
                Vertex { pos: [ -0.5, -0.6 ] },
                Vertex { pos: [ 0.5, -0.6 ] },

                Vertex { pos: [ 1., 0.5 ] },
                Vertex { pos: [ 0.5, -0.5 ] },
                Vertex { pos: [ 1.5, -0.5 ] },

                Vertex { pos: [ -1., 0.5 ] },
                Vertex { pos: [ -1.5, -0.5 ] },
                Vertex { pos: [ -0.5, -0.5 ] },
                ],
            )
    }

    pub fn line(graphics: &mut Graphics) -> ShapeDrawer {
        ShapeDrawer::new(
            graphics,
            gfx::PrimitiveType::LineStrip,
            &[
                Vertex { pos: [ 0.0, 0.0 ] },
                Vertex { pos: [ 1.0, 0.0 ] },
                ],
            )
    }

    pub fn planet(graphics: &mut Graphics) -> ShapeDrawer {
        let shape = mesh_oval(1.0,1.0,50);
        
        ShapeDrawer::new(
            graphics,
            gfx::PrimitiveType::TriangleStrip,
            shape.as_ref(),
        )
    }

    pub fn draw(
        &mut self,
        size     : f32,
        color    : color::Color,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
        ) {
        let params = Params {
            transform: *transform.as_ref(),
            size     : size,
            color    : color,
            _r       : PhantomData,
        };

        graphics.draw(
            &self.batch,
            &params,
            );
    }
}

/// convenience function to build planet mesh
fn mesh_oval (w: f32, h: f32, n: u8) -> Vec<Vertex> {
    let t = 2.0 * (PI as f32) / n as f32;
    let hw = w / 2.0;
    let hh = h / 2.0;
    
    let mut points: Vec<Vertex> = vec!();
    for i in 0..n+1 {
        points.push(Vertex { pos: [0.0,0.0] });
        points.push(Vertex { pos: [hw * (t*i as f32).cos(),
                                   hh * (t*i as f32).sin()] });
    }
    
    points
}
