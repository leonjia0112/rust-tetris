extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;

pub fn main() {
	// Initialize sdl context so it is running
	let sdl_context = sdl2::init().expect("SDL initialization fail");
	
	// get the video subsystem to display on the screen
	let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

	// create window 
	let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
		.position_centered()
		.opengl()
		.build()
		.expect("Failed to create window");

	// convert the window to canvas for display
	let mut canvas = window.into_canvas().build().expect("Failed to convert window into canvas");

	// draw on window
	canvas.set_draw_color(Color::RGB(255, 0, 0));
	canvas.clear();
	canvas.present();

	// getting events
	let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

	// infinte loop listening on event
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
					break 'running
				},
				_ => {}

			}
		}

		sleep(Duration::new(0, 1_000_000_000u32/60));
	}
}