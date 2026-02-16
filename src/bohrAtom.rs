use crate::winsdl::Winsdl;
use sdl2::event::Event;                                                                    // Import the Winsdl struct and the Event from the sdl2 crate

pub struct BohrAtom {
	position: (usize, usize),
	charge: f32,

}

impl BohrAtom {
	pub fn new(position: (usize, usize), charge: f32) -> Self {
		Self {
			position,
			charge,
		}
	}

	pub fn draw(&self) {
		// Use self.position and self.charge here to set GL coordinates
    unsafe {
			// For example, using the charge to affect the color
      gl::ClearColor(0.1, 0.1, 0.1, 1.0); 
        }
    }
}
