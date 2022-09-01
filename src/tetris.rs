mod geometry;
mod map;
mod random;
mod score;
mod tetrimino;

use {
	crate::game::Msg,
	map::{
		Map,
		MapIter,
	},
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
		let tetrimino = None;
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
		// println!("{:?}", self.tetrimino);
		// generating a new one tetrimino
		if self.tetrimino.is_none() {
			// println!("new tetrimino");
			self.tetrimino = Some(self.generator.gen());
		}			

		// unwrap won't panic
		let tetrimino = self.tetrimino.as_mut().unwrap();

		// trying to descent existing tetrimino
		if !tetrimino.shift(Dwn, &mut self.map) {
			// leave tetrimino's corpse on the map
			self.map.kill();
			self.tetrimino = None;

			// burn completed lines
			let lines = self.map.burn();
			// increase lines, score, level
			self.lines += lines;
			self.score += match lines {
				0 => 0,
				1 => 40 * (self.level + 1),
				2 => 100 * (self.level + 1),
				3 => 300 * (self.level + 1),
				4 => 1200 * (self.level + 1),
				_ => unreachable!(),
			};

			if self.lines >= LEVEL_LINES[self.level] {
				self.level += 1;
			}
		}
	}

	// handling the given event related to the game mechanics
	pub fn react_to(&mut self, msg: Msg) {
		// very soon a new one tetrimino will be generated
		if self.tetrimino.is_none() {
			return;
		}
		// println!("{:?}\n", msg);

		// unwrap won't panic
		let tetrimino = self.tetrimino.as_mut().unwrap();

		match msg {
            Msg::Proceed => self.proceed(),
            Msg::ShiftRgt =>
            	_ = tetrimino.shift(Rgt, &mut self.map),
            Msg::ShiftLft =>
            	_ = tetrimino.shift(Lft, &mut self.map),
            Msg::Clockwise =>
            	_ = tetrimino.rotate(true, &mut self.map),
            Msg::Counterclockwise =>
            	_ = tetrimino.rotate(false, &mut self.map),
            _ => unreachable!(),
        };
	}

	pub fn print_map(&self) {
		for (coord, color) in self.map.iter() {
			print!("{:?} ", color);
		}
		println!("\n");
	}

	// shows how long will tetrimino hang without descending
	pub fn level_time(&self) -> Duration {
		Duration::from_millis(LEVEL_TIMES[min(self.level, 9)])
	}

	pub fn map_iter(&self) -> MapIter {
		self.map.iter()
	}
}
