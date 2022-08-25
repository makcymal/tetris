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
		geometry::{
			Coord,
			Segment,
			Rectangle,
			RectangleIter
		},
		tetrimino::Tetrimino,
	},
};


pub struct Map {
	// two-dim array of Color's
	// if color is Non then block is empty else it's not
	map: [[Color; 16]; 10],
	// alive tetrimino that will be moved
	tetrimino: Vec<Coord<usize>>,
	// bg is selected randomly by tetrimino generator
	bg: Color,
}

impl Map {
	pub fn new(bg: Color) -> Map {
		Map {
			map: [[Non; 16]; 10],
			tetrimino: vec!(),
			bg,
		}
	}

	// returns coord of lowermost middle point above (not on) the map
	pub fn top() -> Coord<i8> {
		Coord {
			x: 4,
			y: 16,
		}
	}

	// validate tetrimino's pos and if it's valid puts it on map
	pub fn put(&mut self, tetrimino: &Tetrimino) -> bool {
		// clear prev tetrimino's pos
		self.kill();

		// blocks validated for putting on the map
		let mut validated: Vec<Coord<usize>> = vec!();

		for block in tetrimino.iter() {
			let x = block.x as usize;
			let y = block.y as usize;

			// if block is invalid
			if x < 0 || 9 < x || y < 0 ||
			   y < 16 && self.map[x][y] != Non {
				for block in validated {
					self.map[block.x][block.y] = Non;
				}
				return false
			}

			// if it's valid for putting on the map
			if y < 16 {
				validated.push(Coord::from((x, y)));
			}
		}

		// it's reachable only if all blocks are valid
		for block in validated {
			self.map[block.x][block.y] = tetrimino.color;
		}

		true
	}

	// leaves tetrimino as blocks in the map
	pub fn kill(&mut self) {
		for block in &self.tetrimino {
			self.map[block.x][block.y] = Non;
		}
	}

	// burns completes lines, returns (burned lines, earned score)
	pub fn burn(&mut self, level: usize) -> (usize, usize) {
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

		(serial, score)
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

	pub fn iter(&self) -> MapIter {
		MapIter {
			curr: Coord { x: 0, y: 0 },
			map: &self.map,
			bg: self.bg,
		}
	}
}


// iterating first by x then by y
// returns colors of blocks on the map
pub struct MapIter<'a> {
	curr: Coord<usize>,
	map: &'a [[Color; 16]; 10],
	bg: Color,
}

impl Iterator for MapIter<'_> {
	type Item = (Coord<usize>, Color);

	fn next(&mut self) -> Option<Self::Item> {
		if self.curr.y > 15 {
		   	return None
		}

		let coord = self.curr;

		let color = 
			if self.map[coord.x][coord.y] == Non {
				self.bg
			} else {
				self.map[coord.x][coord.y]
			};

		if coord.x == 9 {
			self.curr.y += 1;
			self.curr.x = 0;
		} else {
			self.curr.x += 1;
		}

		Some((coord, color))
	}
}
