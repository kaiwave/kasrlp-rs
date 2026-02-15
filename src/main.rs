use crate::winsdl::Winsdl;
use sdl2::event::Event;

mod winsdl;

fn main(){
	let mut winsdl = Winsdl::new(800, 600).unwrap();

	'running: loop{
		for event in winsdl.event_pump.poll_iter() {
			match event {
				Event::Quit {..} => break 'running,
				_ => {},
			}
		}

		unsafe {
			gl::ClearColor(1, 0.1, 0.1, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		winsdl.window.gl_swap_window();
	}
}