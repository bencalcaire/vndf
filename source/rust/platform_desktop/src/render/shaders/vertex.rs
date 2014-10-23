use gfx;


pub static FIXED_SIZE_BILLBOARD: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3 position;
		uniform mat4 transform;
		uniform vec2 size;
		uniform vec2 offset;
		uniform vec2 screen_size;

		in vec3 vertex;
		in vec2 tex_coord;

		out vec2 texture_coordinate;
		out vec2 point;

		void main() {
			gl_Position = transform * vec4(position, 1.0);
			gl_Position /= gl_Position.w;
			gl_Position.xy += vertex.xy * size / screen_size;
			gl_Position.xy += offset * 2 / screen_size;

			texture_coordinate = tex_coord;
			point = vertex.xy;
		}
	"
};

pub static LINE: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3 center;
		uniform vec3 position;
		uniform mat4 transform;

		in vec3 vertex;

		void main() {
			vec3 point = position - vec3(0.0, 0.0, position.z - center.z) * vertex.z;
			gl_Position = transform * vec4(point, 1.0);
		}
	"
};

pub static RINGS: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform mat4  transform;
		uniform float radius;

		in vec3 vertex;

		out vec2 point;

		void main() {
			gl_Position = transform * vec4(vertex * radius, 1.0);

			point = vertex.xy;
		}
	"
};

pub static SCALED_BILLBOARD: gfx::ShaderSource = shaders! {
	GLSL_150: b"
		#version 150 core

		uniform vec3  position;
		uniform float radius;
		uniform mat4  transform;
		uniform vec3  camera_right_world;
		uniform vec3  camera_up_world;

		in vec3 vertex;

		out vec2 point;

		void main() {
			vec3 vertex_world =
				position
				+ camera_right_world * vertex.x * radius
				+ camera_up_world * vertex.y * radius;

			gl_Position = transform * vec4(vertex_world, 1.0);
			point = vertex.xy;
		}
	"
};
