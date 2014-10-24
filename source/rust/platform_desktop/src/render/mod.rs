use cgmath::Matrix4;
use gfx;


pub mod renderer;

mod drawers;
mod shaders {
	pub mod fragment;
	pub mod vertex;
}
mod texture;


type Graphics  = gfx::Graphics<gfx::GlDevice, gfx::GlCommandBuffer>;
type Transform = Matrix4<f32>;


#[vertex_format]
pub struct Vertex {
	vertex   : [f32, ..3],
	tex_coord: [f32, ..2],
}

impl Vertex {
	fn new(position: [f32, ..3], tex_coord: [f32, ..2]) -> Vertex {
		Vertex {
			vertex   : position,
			tex_coord: tex_coord,
		}
	}
}
