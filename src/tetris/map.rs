// height of 16 blocks, width of 10 blocks
//
// 		y │			 _
// 		  │			|+| - top, here tetriminos spawn, not visible
//	   15 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//	   14 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//	   13 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//	   12 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//	   11 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//	   10 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		9 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		8 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		7 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		6 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		5 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		4 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		3 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		2 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		1 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		0 ┼ |‾|‾|‾|‾|‾|‾|‾|‾|‾|‾|
//		  │	 ‾ ‾ ‾ ‾ ‾ ‾ ‾ ‾ ‾ ‾
// 		  └──┼─┼─┼─┼─┼─┼─┼─┼─┼─┼─────
//			 0 1 2 3 4 5 6 7 8 9	x

use {
	crate::{
		color::Color::{
			self,
			*,
		},
	},
	super::{
		geom::Point,
		tetrimino::Tetrimino,
	},
};


// two-dim array of Color's
// if color is Non then block is empty else it isn't
pub struct Map {
	map: [[Color; 16]; 10],
	// bg is selected randomly by tetrimino generator
	bg: Color,
}


impl Map {
	pub fn new(bg: Color) -> Map {
		Map {
			map: [[Non; 16]; 10],
			bg,
		}
	}

	// returns coord of lowermost middle point above (not on) the map
	pub fn top() -> Point {
		Point {
			x: 4,
			y: 16,
		}
	}

	// returned bool means can such tetrimino be placed on the map or not
	pub fn validate(&self, tetrimino: &mut Tetrimino) -> bool {
		for block in tetrimino.iter() {
			if self.map[block.x as usize][block.y as usize] != Non {
				return false
			}
		}

		true
	}

	// leaves tetrimino as blocks in the map
	pub fn kill(&mut self, tetrimino: Tetrimino) {
		for block in tetrimino.iter() {
			self.map[block.x as usize][block.y as usize] =
				tetrimino.color;
		}
		// now tetrimino value will be dropped
	}

	// burns completes lines, returns (burned lines, earned score)
	pub fn burn(&mut self, level: usize) -> (u8, usize) {
		let mut score = 0;
		let mut serial = 0;
		let mut prev_completed = false;

		// iterating by lines up from the bottom
		'lines: for y in 0..16 {
			// iterating by blocks in line
			for x in 0..10 {
				// if the line isn't completed
				if self.map[x][y] == Non {
					// it's time to burn
					if prev_completed {
						self.burn_from(y, serial);

						score += match serial {
							1 => 40 * level,
							2 => 100 * level,
							3 => 300 * level,
							4 => 1200 * level,
							_ => unreachable!(),
						};
						prev_completed = false;
					}

					// it doesn't make sense to iterate further
					continue 'lines;
				}
			}

			// if the line is completed
			// it's time to update range in which lines will be burnt
			if prev_completed {
				serial += 1;
			} else {
				serial = 0;
			}

			prev_completed = true;
		}

		(serial as u8, score)
	}

	// burns line of given number starting with from
	fn burn_from(&mut self, from: usize, serial: usize) {
		for y in (from + serial)..16 {
			for x in 0..10 {
				self.map[x][y - serial] = self.map[x][y];
			}
		}

		for y in (16 - serial)..16 {
			for x in 0..10 {
				self.map[x][y] = Non;
			}
		}
	}
}
