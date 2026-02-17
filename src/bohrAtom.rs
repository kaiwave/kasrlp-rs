use crate::winsdl::Winsdl;
use sdl2::event::Event;                                                                     // Import the Winsdl struct and the Event from the sdl2 crate
use gl::*;                                                                                  // Import gl stuffs

pub struct BohrAtom {
	position: (usize, usize),
	charge: f32,                                                                              // Declare the BohrAtom struct with a position and a charge 
}

impl BohrAtom {
	pub fn new(position: (usize, usize), charge: f32) -> Self {
		Self {
			position,
			charge,                                                                               // Store the position and charge in the struct, and return a new instance of BohrAtom with those values
		}
	}

	pub fn draw(&self) {
		                                                                                        // Use self.position and self.charge here to set GL coordinates
		type Vertex = [f32; 3];
		const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];   // Testing triangle drawing

		const VERT_SHADER: &str = r#"#version 330 core
		layout (location = 0) in vec3 pos;
		void main() {
			gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
		}
		"#;                                                                                     // Vertex shader source code, which takes in a position and sets the gl_Position to that position

		const FRAG_SHADER: &str = r#"#version 330 core
		out vec4 final_color;
		void main() {
			final_color = vec4(1.0, 0.5, 0.2, 1.0);
		}
		"#;                     																					                     	// Fragment shader source code, which sets the final color to a specific value

    unsafe {
			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao);                                                     // Generate a vertex array object and assert that it was created successfully (not zero)
			gl::BindVertexArray(vao);
			assert_ne!(vao, 0);                                                                   // This is an object that collects bits of stuff, and we need it to buffer data and describe it

			let mut vbo = 0;
			gl::GenBuffers(1, &mut vbo);                                                          // Generate a vertex buffer object and assert that it was created successfully (not zero)
			assert_ne!(vbo, 0);                                                                   // This is an object that holds the actual vertex data, and we need it to buffer the data for drawing

			BindBuffer(ARRAY_BUFFER, vbo);                                                        // Bind the VBO to the ARRAY_BUFFER target, which means that subsequent buffer operations will affect this VBO

			BufferData(
				ARRAY_BUFFER,
				size_of_val(&VERTICES) as isize,
				VERTICES.as_ptr().cast(),
				STATIC_DRAW,
			);

			VertexAttribPointer(
				0,
				3,
				FLOAT,
        FALSE,
				size_of::<Vertex>().try_into().unwrap(),
				0 as *const _,
			);
			EnableVertexAttribArray(0);

			let vertex_shader = CreateShader(VERTEX_SHADER);
			assert_ne!(vertex_shader, 0);

			ShaderSource(
				vertex_shader,
				1,
				&(VERT_SHADER.as_bytes().as_ptr().cast()),
				&(VERT_SHADER.len().try_into().unwrap()),
			);

			CompileShader(vertex_shader);

			let mut success = 0;
			GetShaderiv(vertex_shader, COMPILE_STATUS, &mut success);

			if success == 0 {
				let mut v: Vec<u8> = Vec::with_capacity(1024);
				let mut log_len = 0_i32;
				GetShaderInfoLog(
					vertex_shader,
					1024,
					&mut log_len,
					v.as_mut_ptr().cast(),
				);
				v.set_len(log_len.try_into().unwrap());
				panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
			}

			let fragment_shader = CreateShader(FRAGMENT_SHADER);
			assert_ne!(fragment_shader, 0);

			ShaderSource(
				fragment_shader,
				1,
				&(FRAG_SHADER.as_bytes().as_ptr().cast()),
				&(FRAG_SHADER.len().try_into().unwrap()),
			);
			CompileShader(fragment_shader);

			let mut success = 0;
			GetShaderiv(fragment_shader, COMPILE_STATUS, &mut success);
			if success == 0 {
				let mut v: Vec<u8> = Vec::with_capacity(1024);
				let mut log_len = 0_i32;
				GetShaderInfoLog(
					fragment_shader,
					1024,
					&mut log_len,
					v.as_mut_ptr().cast(),
				);
			v.set_len(log_len.try_into().unwrap());
			panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
			}

			let shader_program = CreateProgram();
			AttachShader(shader_program, vertex_shader);
			AttachShader(shader_program, fragment_shader);
			LinkProgram(shader_program);

			let mut success = 0;
			GetProgramiv(shader_program, LINK_STATUS, &mut success);
			if success == 0 {
				let mut v: Vec<u8> = Vec::with_capacity(1024);
				let mut log_len = 0_i32;
				GetProgramInfoLog(
					shader_program,
					1024,
					&mut log_len,
					v.as_mut_ptr().cast(),
				);
			v.set_len(log_len.try_into().unwrap());
			panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
			}

			DeleteShader(vertex_shader);
			DeleteShader(fragment_shader);

      Clear(COLOR_BUFFER_BIT);

			UseProgram(shader_program);

			DrawArrays(TRIANGLES, 0, 3);
    }
  }
}
