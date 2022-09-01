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
	drawed: Vec<Coord<usize>>,
	// bg is selected randomly by tetrimino generator
	bg: Color,
}

impl Map {
	pub fn new(bg: Color) -> Map {
		Map {
			map: [[Non; 16]; 10],
			drawed: vec!(),
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
		for tile in &self.drawed {
			self.map[tile.x][tile.y] = Non;
		}

		// drawable tiles
		let mut drawable: Vec<Coord<usize>> = vec!();

		for tile in tetrimino.iter() {
			if tile.x < 0 || tile.y < 0 {
				for tile in &self.drawed {
			   		self.map[tile.x][tile.y] = tetrimino.color;
			   	}
			   	return false;
			}

			let x = tile.x as usize;
			let y = tile.y as usize;

			// if tile is invalid
			if x < 0 || 9 < x || y < 0 ||
			   y < 16 && self.map[x][y] != Non {
			   	for tile in &self.drawed {
			   		self.map[tile.x][tile.y] = tetrimino.color;
			   	}
				return false
			}

			// if it's drawable
			if y < 16 {
				drawable.push(Coord::from((x, y)));
			}
		}

		// it's reachable only if all blocks are valid
		self.drawed.clear();
		for tile in drawable {
			self.map[tile.x][tile.y] = tetrimino.color;
			self.drawed.push(tile);
		}

		true
	}

	// leaves tetrimino as blocks in the map
	pub fn kill(&mut self) {
		self.drawed.clear();
	}

	// burns completes lines, returns (burned lines, earned score)
	pub fn burn(&mut self) -> usize {
		let mut lines = 0;
		let mut serial = 0;

		for y in (0..16).rev() {
			if self.line_completed(y) {
				serial += 1;
			} else {
				self.burn_from(y + 1, serial);
				lines += serial;
				serial = 0;
			}
		}

		self.burn_from(0, serial);
		lines += serial;
		lines
	}

	// burns line of serial number in [line, line + serial)
	fn burn_from(&mut self, line: usize, serial: usize) {
		if serial == 0 { return }

		for y in (line + serial)..16 {
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

	fn line_completed(&self, y: usize) -> bool {
		for x in 0..9 {
			if self.map[x][y] == Non {
				return false;
			}
		}
		true
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
