use std::ffi::CString;
use std::ptr;
use std::slice;

use libc::c_ulong;
use nalgebra::Vec2;
use freetype::ffi::{
	FT_Face,
	FT_Get_Char_Index,
	FT_GlyphSlot,
	FT_Init_FreeType,
	FT_Library,
	FT_LOAD_DEFAULT,
	FT_Load_Glyph,
	FT_New_Face,
	FT_Render_Glyph,
	FT_RENDER_MODE_NORMAL,
	FT_Set_Pixel_Sizes
};


pub struct Font {
	pub font_face: FT_Face,
}

impl Font {
	pub fn load(size: u32) -> Font {
		Font {
			font_face: init_font_face(size),
		}
	}

	pub fn glyph(&self, c: char) -> Option<Glyph> {
		let glyph_slot = match load_glyph_slot(self.font_face, c) {
			Some(glyph_slot) => glyph_slot,
			None             => return None,
		};

		Some(make_glyph(glyph_slot))
	}
}


pub struct Glyph {
	pub data   : Vec<u8>,
	pub size   : Vec2<f32>,
	pub offset : Vec2<f32>,
	pub advance: Vec2<f32>,
}


fn init_font_face(size: u32) -> FT_Face {
	unsafe {
		let mut freetype: FT_Library = ptr::null_mut();
		let init_error = FT_Init_FreeType(&mut freetype);
		assert!(init_error == 0);

		let mut font_face: FT_Face = ptr::null_mut();
		let face_error = FT_New_Face(
				freetype,
				CString::new(b"source/assets/UbuntuMono-B.ttf".as_ref())
					.unwrap_or_else(|e| panic!("Error creating CString: {}", e))
					.as_ptr(),
				0,
				&mut font_face
		);
		assert!(face_error == 0);

		let pixel_error = FT_Set_Pixel_Sizes(
			font_face,
			0,
			size,
		);
		assert!(pixel_error == 0);

		font_face
	}
}

fn load_glyph_slot(font_face: FT_Face, c: char) -> Option<FT_GlyphSlot> {
	unsafe {
		let glyph_index = match FT_Get_Char_Index(font_face, c as c_ulong) {
			0     => return None, // undefined character code
			index => index,
		};

		let glyph_error = FT_Load_Glyph(
			font_face,
			glyph_index,
			FT_LOAD_DEFAULT,
		);
		assert!(glyph_error == 0);

		let render_error = FT_Render_Glyph(
			(*font_face).glyph,
			FT_RENDER_MODE_NORMAL
		);
		assert!(render_error == 0);

		Some((*font_face).glyph)
	}
}

fn make_glyph(glyph_slot: FT_GlyphSlot) -> Glyph {
	unsafe {
		let ref bitmap = (*glyph_slot).bitmap;

		Glyph {
			data:
				slice::from_raw_parts(
					bitmap.buffer as *mut u8,
					(bitmap.width * bitmap.rows) as usize,
				)
				.to_vec(),
			size: Vec2::new(
				bitmap.width as f32,
				bitmap.rows as f32,
			),
			offset: Vec2::new(
				(*glyph_slot).bitmap_left as f32,
				(*glyph_slot).bitmap_top as f32 - bitmap.rows as f32
			),
			advance: Vec2::new(
				(*glyph_slot).advance.x as f32 / 64.0,
				(*glyph_slot).advance.y as f32 / 64.0
			),
		}
	}
}
