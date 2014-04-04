use gl;

use common::vec::Vec2;

use components::Visual;
use entities::Components;
use ui::{Font, Texture, Textures, Window};


pub struct Renderer {
	screen_width : f64,
	screen_height: f64,

	textures: Textures,
	font    : Font
}

impl Renderer {
	pub fn init(window: &Window, textures: Textures, font: Font) -> Renderer {
		gl::LoadIdentity();
		gl::Ortho(
			0.0,
			window.width as f64,
			0.0,
			window.height as f64,
			-100.0,
			100.0);

		Renderer {
			screen_width : window.width as f64,
			screen_height: window.height as f64,

			textures: textures,
			font    : font
		}
	}

	pub fn render(&self,
		window   : &Window,
		camera   : Vec2,
		positions: &Components<Vec2>,
		visuals  : &Components<Visual>) {

		gl::Clear(gl::COLOR_BUFFER_BIT);
		gl::Color4d(1.0, 1.0, 1.0, 1.0);

		gl::PushMatrix();
		{
			gl::Translated(
				self.screen_width / 2.0 - camera.x,
				self.screen_height / 2.0 - camera.y,
				0.0);

			for (id, &position) in positions.iter() {
				let texture = self.textures.get(&visuals.get(id).texture);

				let draw_position = position - texture.size * 0.5;
				draw_texture(draw_position, texture);
			}
		}
		gl::PopMatrix();

		self.draw_ui_overlay();

		window.swap_buffers();

		match gl::GetError() {
			gl::NO_ERROR => (),
			error @ _    => fail!("OpenGL error ({})", error)
		}
	}

	fn draw_ui_overlay(&self) {
		self.draw_text(
			Vec2 { x: 20.0, y: 40.0 },
			"Set attitude with the left and right cursor keys");
		self.draw_text(
			Vec2 { x: 20.0, y: 20.0 },
			"Commit maneuver with Enter");
	}

	fn draw_text(&self, mut position: Vec2, text: &str) {
		for c in text.chars() {
			let glyph   = self.font.get(c);
			let texture = self.textures.get(&glyph.texture_id);

			draw_texture(position + glyph.offset, texture);

			position = position + glyph.advance;
		}
	}
}

fn draw_texture(position: Vec2, texture: &Texture) {
	gl::BindTexture(
		gl::TEXTURE_2D,
		texture.name);

	gl::PushMatrix();
	{
		gl::Translated(
			position.x,
			position.y,
			0.0);

		gl::Begin(gl::TRIANGLE_STRIP);
		{
			gl::TexCoord2d(
				1.0,
				0.0);
			gl::Vertex3d(
				texture.size.x,
				texture.size.y,
				0.0);

			gl::TexCoord2d(
				1.0,
				1.0);
			gl::Vertex3d(
				texture.size.x,
				0.0,
				0.0);

			gl::TexCoord2d(
				0.0,
				0.0);
			gl::Vertex3d(
				0.0,
				texture.size.y,
				0.0);

			gl::TexCoord2d(
				0.0,
				1.0);
			gl::Vertex3d(
				0.0,
				0.0,
				0.0);
		}
		gl::End();
	}
	gl::PopMatrix();
}
