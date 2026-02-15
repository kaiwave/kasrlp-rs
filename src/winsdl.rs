use sdl2::{Sdl, video::{Window, GLContext, SwapInterval}, EventPump};                      // Everything we should need from SDL2

pub struct Winsdl {
	pub sdl: Sdl,
	pub window: Window,
	pub gl_context: GLContext,
	pub gl: (),
	pub event_pump: EventPump,
}                                                                                           // Init the struct for use in main

impl Winsdl {
	pub fn new(width:usize, height:usize) ->  Result<Self, &'static str> {                    // Create func which takes w/h as input and returns a result with the struct or an error string
		let sdl = sdl2::init().unwrap(); 
		let video_subsystem = sdl.video().unwrap();                                             // Set up SDL and the video subsystem, unwrap to panic if it fails

		let gl_attr = video_subsystem.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);                              // Set the OpenGL attributes to use the core profile and version 3.3
		gl_attr.set_context_version(3, 3);                                                      // Will be most compatible with other machines although its probably unlikely I'll run it on anything else anyway

		let window = video_subsystem
			.window("ATOM", width as u32, height as u32)
			.opengl()
			.build()                        
			.unwrap();                                                                            // Create the window with the specified width and height, set it to use OpenGL, and unwrap to panic if it fails

		let gl_context = window.gl_create_context().unwrap();
		let gl = gl::load_with(|s| {
			video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void                 // Load the OpenGL function pointers using the video subsystem's gl_get_proc_address function, and cast the result to a raw pointer
		});         

		window
			.subsystem()
			.gl_set_swap_interval(SwapInterval::VSync)
			.unwrap();                                                                            // Set the swap interval to VSync, and unwrap to panic if it fails

		let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();                       			// Create the event pump for handling events, and unwrap to panic if it fails

		Ok(Winsdl {
			sdl,
			window,
			gl_context,
			gl,
			event_pump,
		})                                                                                      // Return the struct with all the initialized fields
	}
}