extern crate sdl2;

use self::sdl2::video::{WindowPos, Window, OPENGL};
use self::sdl2::event::{Event, poll_event};
use self::sdl2::surface::{Surface};

pub fn show (width: u32, height: u32) {
	sdl2::init(sdl2::INIT_VIDEO);

	let window = match sdl2::video::Window::new("image", WindowPos::PosCentered, WindowPos::PosCentered, width as isize, height as isize, OPENGL) {
		Ok(window) => window,
		Err(err) => panic!("failed to create window: {}", err)
	};
	
	let surface = match Surface::from_bmp(&Path::new("out.bmp")) {
		Ok(surface) => surface,
		Err(err) => panic!("failed to load surface: {}", err)
	};
	
	let screen = match window.get_surface() {
		Ok(s) => s,
		Err(err) => panic!("failed to get window surface: {}", err)
	};
	
	screen.blit(&surface, None, None);
	
	window.update_surface();
	'event : loop {
		match poll_event() {
			sdl2::event::Event::Quit(_) => break 'event,
			_ => {}
		}
	}
	sdl2::quit();	
}
