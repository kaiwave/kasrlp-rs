mod bohrAtom;
mod winsdl;
use gl::*;
use sdl2::event::Event;

fn main() {
  let mut winsdl = winsdl::Winsdl::new(800, 600).unwrap();
  let hydrogen = bohrAtom::BohrAtom::new((400, 300), 1.0);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            if let Event::Quit {..} = event { break 'running }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // The atom draws itself onto the existing context
        hydrogen.draw();

        winsdl.window.gl_swap_window();
    }
}