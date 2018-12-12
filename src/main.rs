extern crate sdl2;
extern crate rand;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

use std::time::Duration;
use std::thread::sleep;
use sdl2::video::{Window, WindowContext};
use sdl2::render::Canvas;

#[derive(Clone, Copy)]
enum TextureColor{
	Green,
	Blue,
}

// 0 0 0 0
// 0 0 0 0
// 1 1 0 0
// 1 1 0 0
type Piece = Vec<Vec<u8>>;

type States = Vec<Piece>;

struct Tetrimino {
	states: States,
	x: isize,
	y: usize,
	current_state: u8,
}

trait TetriminoGenerator {
	fn new() -> Tetrimino;
}

struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![1, 1, 1, 1],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![0, 1, 0, 0],
						 	 vec![0, 1, 0, 0],
						 	 vec![0, 1, 0, 0],
						 	 vec![0, 1, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

struct TetriminoS;

impl TetriminoGenerator for TetriminoS {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![0, 5, 5, 0],
						 	 vec![5, 5, 0, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![5, 0, 0, 0],
						 	 vec![5, 5, 0, 0],
						 	 vec![0, 5, 0, 0],
						 	 vec![0, 0, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

struct TetriminoZ;

impl TetriminoGenerator for TetriminoZ {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![0, 6, 0, 0],
						 	 vec![6, 6, 0, 0],
						 	 vec![6, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![6, 6, 0, 0],
						 	 vec![0, 6, 6, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

struct TetriminoO;

impl TetriminoGenerator for TetriminoO {
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![4, 4, 0, 0],
						 	 vec![4, 4, 0, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

struct TetriminoJ;

impl TetriminoGenerator for TetriminoJ{
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![2, 2, 2, 0],
						 	 vec![0, 0, 2, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![0, 2, 0, 0],
						 	 vec![0, 2, 0, 0],
						 	 vec![2, 2, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![2, 0, 0, 0],
						 	 vec![2, 2, 2, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![2, 2, 0, 0],
						 	 vec![2, 0, 0, 0],
						 	 vec![2, 0, 0, 0],
						 	 vec![0, 0, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

struct TetriminoL;

impl TetriminoGenerator for TetriminoL{
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![0, 0, 3, 0],
						 	 vec![3, 3, 3, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![3, 3, 0, 0],
						 	 vec![0, 3, 0, 0],
						 	 vec![0, 3, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![3, 3, 3, 0],
						 	 vec![3, 0, 0, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![3, 0, 0, 0],
						 	 vec![3, 0, 0, 0],
						 	 vec![3, 3, 0, 0],
						 	 vec![0, 0, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}

struct TetriminoT;

impl TetriminoGenerator for TetriminoT{
	fn new() -> Tetrimino {
		Tetrimino {
			states: vec![
						vec![vec![0, 7, 0, 0],
						 	 vec![7, 7, 7, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![7, 0, 0, 0],
						 	 vec![7, 7, 0, 0],
						 	 vec![7, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![7, 7, 7, 0],
						 	 vec![0, 7, 0, 0],
						 	 vec![0, 0, 0, 0],
						 	 vec![0, 0, 0, 0]],
						vec![vec![0, 7, 0, 0],
						 	 vec![7, 7, 0, 0],
						 	 vec![0, 7, 0, 0],
						 	 vec![0, 0, 0, 0]]],
			x: 4,
			y: 0,
			current_state: 0,
		}
	}
}


fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, 
	texture_creator: &'a TextureCreator<WindowContext>, 
	color: TextureColor, size:u32) -> Option<Texture<'a>> {
	if let Ok(mut square_texture) = texture_creator.create_texture_target(None, size, size) {
		canvas.with_texture_canvas(&mut square_texture, |texture| {
			match color {
				TextureColor::Green => texture.set_draw_color(Color::RGB(0, 255, 0)),
				TextureColor::Blue => texture.set_draw_color(Color::RGB(0, 0, 255)),
			}
			texture.clear();
		}).expect("Failed to color a texture");
		Some(square_texture)
	} else {
		None
	}
}

fn create_new_tetrimino() -> Tetrimino {
	static mut PREV: u8 = 7;
	let mut rand_nb = rand::random::<u8>() % 7;
	if unsafe { PREV } == rand_nb {
		rand_nb = rand::random::<u8>() % 7;
	}

	unsafe { PREV = rand_nb; }
	match rand_nb {
		0 => TetriminoI::new(),
		1 => TetriminoJ::new(),
		2 => TetriminoL::new(),
		3 => TetriminoO::new(),
		4 => TetriminoS::new(),
		5 => TetriminoZ::new(),
		6 => TetriminoT::new(),
		_ => unreachable!(),
	}
}

impl Tetrimino {
	fn rotate(&mut self) {
		self.current_state += 1;
		if self.current_state as usize >= self.states.len() {
			self.current_state = 0;
		}
	}

	fn test_postion(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool {
		for decal_y in 0..4 {
			for decal_x in 0..4 {
				let x = x + decal_x;
				if self.states[tmp_state][decal_y][decal_x as usize] != 0 
					&& (y + decal_y >= game_map.len() 
					|| x < 0 
					|| x as usize >= game_map[y + decal_y].len() 
					|| game_map[y + decal_y] [x as usize] != 0 )
				{
					return false;
				}
			}
		}
		true
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