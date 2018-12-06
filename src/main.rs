extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

use std::time::Duration;
use std::thread::sleep;
use sdl2::video::{Window, WindowContext};

#[derive(Clone, Copy)]
enum TextureColor{
	Green,
	Blue,
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, 
	texture_creator: &'a TextureCreator<WindowContext>, 
	color: TextureColor, size:u32) -> Option<Texture<'a>> {
	if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
		canvas.with_texture_canvas(&mut square_texture, |texture| {
			match color {
				TexutreColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
				TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
			}
			texture.clear();
		}).expect("Failed to color a texture");
		Some(square_texture)
	} else {
		None
	}
}


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

	let texture_creator: TextureCreator<_> = canvas.texture_creator();

	const TEXTURE_SIZE: u32 = 32;

	let mut square_texture: Texture = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
		.expect("Failed to create a texture");

	canvas.with_texture_canvas(&mut square_texture, |texture| {
		texture.set_draw_color(Color::RGB(0, 255, 0));
		texture.clear();
	}).expect("Failed to color a texture");

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

		canvas.set_draw_color(Color::RGB(0, 0, 255));
		canvas.clear();

		canvas.copy(
			&square_texture,
			None,		
			Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE)
		)
		.expect("Couldn't copy texture into window.");

		canvas.present();
		
		sleep(Duration::new(0, 1_000_000_000u32/60));
	}

}