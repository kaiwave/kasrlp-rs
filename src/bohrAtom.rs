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

    unsafe {
			let mut vao = 0;
			gl::GenVertexArrays(1, &mut vao);                                                     // Generate a vertex array object and assert that it was created successfully (not zero)
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

      gl::ClearColor(0.1, 0.1, 0.1, 1.0); 
    }
  }
}
