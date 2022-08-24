mod geometry;
mod map;
mod random;
// mod score;
mod tetrimino;

use {
	map::Map,
	geometry::{
		Direction::{
			self,
			*,
		},
	},
	tetrimino::{
		Generator,
		Tetrimino,
	},
	sdl2::{
		keyboard::{
			Keycode,
			Mod,
		},
		event::Event,
	},
	std::{
		time::{
			Duration,
		},
		cmp::min,
	},
};


// number of burned lines to go to next level
const LEVEL_LINES: [usize; 10] =
	[20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
// time in millis after which tetrimino descents
const LEVEL_TIMES: [u64; 10] =
	[1000, 850, 700, 600, 500, 400, 300, 250, 220, 190];


pub struct Tetris {
	level: usize,		// level is counted from zero
	score: usize,
	lines: usize,
	generator: Generator,
	tetrimino: Option<Tetrimino>,
	map: Map,
}

impl Tetris {
	// makes all setup for the game
	pub fn new() -> Tetris {
		let generator = Generator::new();
		let tetrimino = Some(generator.gen());
		let map = Map::new(generator.map_bg());

		Tetris {
			level: 0,
			score: 0,
			lines: 0,
			generator,
			tetrimino,
			map,
		}
	}

	// generating, descenting or killing tetrimino
	pub fn proceed(&mut self) {
		// generating a new one tetrimino
		if self.tetrimino.is_none() {
			self.tetrimino = Some(self.generator.gen());
		}			

		// unwrap won't panic
		let tetrimino = self.tetrimino.as_mut().unwrap();

		// trying to descent existing tetrimino
		if !tetrimino.push(&mut self.map, Dwn) {
			// leave tetrimino's corpse on the map
			self.map.kill();
			self.tetrimino = None;

			// burn completed lines
			let (lines, score) = self.map.burn(self.level + 1);
			// increase lines, score, level
			self.lines += lines;
			self.score += score;

			if self.lines >= LEVEL_LINES[self.level] {
				self.level += 1;
			}
		}
	}

	// handling the given event related to the game mechanics
	pub fn react_to(&mut self, event: Event) {
		// very soon a new one tetrimino will be generated
		if self.tetrimino.is_none() {
			return;
		}

		// unwrap won't panic
		let tetrimino = self.tetrimino.as_mut().unwrap();

		match event {
			// right
			Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
				println!("right");
				tetrimino.push(&mut self.map, Rgt);
			},
			// left
			Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
				println!("left");
				tetrimino.push(&mut self.map, Lft);
			},
			// ctrl + right
			Event::KeyDown { keycode: Some(Keycode::Right),
							 keymod: Mod::LCTRLMOD, .. } => {
				println!("ctrl + right");
				tetrimino.rotate(&mut self.map, true);
			},
			// ctrl + left
			Event::KeyDown { keycode: Some(Keycode::Left),
							 keymod: Mod::LCTRLMOD, .. } => {
				println!("ctrl + left");
				tetrimino.rotate(&mut self.map, false);
			},
			// the rest isn't interesting
			_ => (),
		}
	}

	// shows how long will tetrimino hang without descending
	pub fn level_time(&self) -> Duration {
		Duration::from_millis(LEVEL_TIMES[min(self.level, 9)])
	}
}
