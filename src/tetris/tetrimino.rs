#![allow(non_snake_case)]

use {
	crate::{
		color::{
			Color,
			N_COLORS,
		},
	},
	super::{
		geometry::{
			Direction::{
				self,
				*,
			},
			Coord,
			Segment,
			Rectangle,
		},
		random::{
			non_serial_rnd,
			shuffle_colors,
		},
		map::Map,
	},
};


pub const N_TETRIMINOS: u8 = 7;


#[derive(Debug)]
pub struct Tetrimino {
	// it's |+| in following tetrimino schemes
	center: Coord<i8>,
	// pos of remaining blocks
	shape: [Direction; 3],
	// rel means that each block's pos defines relatively to prev one
	// abs (not rel) means exactly the same but relatively to center
	is_rel: bool,
	// only the same tetriminos may have the same color
	pub color: Color,
}


impl Tetrimino {
	// provides iterating by tetrimino's block coordinates
	pub fn iter(&self) -> TetriminoIter {
		TetriminoIter {
			shape: &self.shape,
			is_rel: self.is_rel,
			index: 0,
			point: self.center,
		}
	}

	// returns bool means was shifting successful or not
	pub fn shift(&mut self, dir: Direction, map: &mut Map) -> bool {
		// shifting to the top is not allowed
		if dir == Top {
			return false;
		}

		// motion vector represented as Coord
		let motion = dir.into();
		
		// checking possibility of motion
		self.center += &motion;

		// put tetrimino on the map if it's possible
		if map.put(self) {
			true
		} else {
			self.center -= &motion;
			false
		}
	}

	// returns bool means was rotation successful or not
	pub fn rotate(&mut self, clockwise: bool, map: &mut Map) -> bool {
		// bounds before rotation
		let old_bounds = self.bounds();

		// rotation itself, regardless to is_rel
		for direction in &mut self.shape {
			direction.rotate(clockwise);
		}

		// bounds after rotation
		let mut new_bounds = self.bounds();

		// moves new_bounds so that it's new_bounds and old_bounds
		// form letter T upside down
		let motion = new_bounds.move_to(&old_bounds);
		// moves tetrimino itself to it's new_bounds
		self.center += &motion;

		// how much new and old bounds differ in width
		let offset_x = 
			(new_bounds.x_axis.len() - old_bounds.x_axis.len()).abs();

		// how much they differ on the left and right side
		// this offsets may differ if offset_x is odd
		let mut offset_lft = offset_x / 2;
		let mut offset_rgt = offset_x - offset_lft;

		// some magic thanks to which narrower bound contains in wider
		if new_bounds.x_axis.len() <= old_bounds.x_axis.len() {
			offset_lft *= -1;
		} else {
			(offset_lft, offset_rgt) = (-offset_lft, offset_rgt);
		}

		// it's appropriate to lift tetrimino up only if it has
		// the less height that before rotation
		let offset_top =
			if new_bounds.y_axis.len() < old_bounds.y_axis.len() {
				old_bounds.y_axis.len() - new_bounds.y_axis.len()
			} else {
				0
			};

		// total offset in which tetrimino after rotation may be shifted
		let offset =
			Rectangle::from((&Coord { x: offset_lft, y: offset_top },
							 &Coord { x: offset_rgt, y: 0		   }));


		// iterating by all appropriate motion vectors
		for motion in offset.iter() {
			self.center += &motion;
		
			// put tetrimino on the map if it's possible
			if map.put(self) {
				return true
			} else {
				self.center -= &motion;
			}
		}

		// it's reachable only if rotation is impossible
		for direction in &mut self.shape {
			self.center -= &motion;	// not from for loop
			direction.rotate(!clockwise);
		}
		
		false
	}

	// returns bounds on the x axis & on the y axis
	// which completely contains tetrimino
	pub fn bounds(&self) -> Rectangle<i8> {
		// now it has only one point 
		let mut rect = Rectangle::from((&self.center, &self.center));

		// iterating by tetrimino block coordinates
		for block in self.iter() {
			rect.extend(&block);
		}

		rect
	}
}


#[derive(Debug)]
pub struct Generator {
	// tetrimino color can be defined as color_picker elem
	// with number which is equal to tetrimino's serial number
	color_picker: [Color; N_COLORS],
}


impl Generator {
	pub fn new() -> Generator {
		Generator {
			color_picker: shuffle_colors(),
		}
	}

	// last color in shuffled is for map bg
	pub fn map_bg(&self) -> Color {
		self.color_picker[N_COLORS - 1]
	}

	// random tetrimino differs from previous one
	pub fn gen(&self) -> Tetrimino {
		let id = non_serial_rnd(N_TETRIMINOS);

		match id {
			0 => self.I(),
			1 => self.J(),
			2 => self.L(),
			3 => self.O(),
			4 => self.S(),
			5 => self.Z(),
			6 => self.T(),
			_ => unreachable!(),  // panics if compiler forgets how to count
		}
	}

	fn I(&self) -> Tetrimino {
		Tetrimino {									//		_
			center: Map::top() + (0, 3),			//	   |+|
			shape: [Dwn, Dwn, Dwn],					//	   |‾|
			is_rel: true,							//	   |‾|
			color: self.color_picker[0],  			//	   |‾|
		}											//		‾
	}

	fn J(&self) -> Tetrimino {
		Tetrimino {
			center: Map::top() + (0, 2),			//		  _
			shape: [Dwn, Dwn, Lft],					//		 |+|
			is_rel: true,				 			//		 |‾|
			color: self.color_picker[1],  			//	   |‾|‾|
		}											//	    ‾ ‾
	}

	fn L(&self) -> Tetrimino {
		Tetrimino {
			center: Map::top() + (0, 2),			//		_
			shape: [Dwn, Dwn, Rgt],					//	   |+|
			is_rel: true,							//	   |‾|
			color: self.color_picker[2],  			//	   |‾|‾|
		}											//		‾ ‾
	}

	fn O(&self) -> Tetrimino {
		Tetrimino {
			center: Map::top() + (0, 1),			//		_ _
			shape: [Rgt, Dwn, Lft],		 			//	   |+| |
			is_rel: true,							//	   |‾|‾|
			color: self.color_picker[3],  			//		‾ ‾
		}
	}

	fn S(&self) -> Tetrimino {
		Tetrimino {
			center: Map::top(),						//		  _ _
			shape: [Rgt, Top, Rgt],					//		_|_|_|
			is_rel: true,							//	   |+| |
			color: self.color_picker[4],  			//		‾ ‾
		}
	}

	fn Z(&self) -> Tetrimino {
		Tetrimino {
			center: Map::top() + (0, 1),			//		_ _
			shape: [Rgt, Dwn, Rgt],					//	   |+| |
			is_rel: true,							//		‾|‾|‾|
			color: self.color_picker[5],  			//		  ‾ ‾
		}
	}

	fn T(&self) -> Tetrimino {
		Tetrimino {
			center: Map::top(),						//		  _
			shape: [Lft, Top, Rgt],					//		_|_|_
			is_rel: false,							//	   | |+| |
			color: self.color_picker[6],  			//		‾ ‾ ‾
		}
	}
}


pub struct TetriminoIter<'a> {
	// exactly the same shape and is_rel as in tetrimino created iter
	shape: &'a [Direction; 3],
	is_rel: bool,
	// number of block whose point will be yielded by next()
	index: usize,
	// point that is needed to compute next
	point: Coord<i8>,
}


impl Iterator for TetriminoIter<'_> {
	type Item = Coord<i8>;

	fn next(&mut self) -> Option<Self::Item> {
		// there are only four blocks
		if self.index >= 4 {
			return None
		}

		// returning center
		if self.index == 0 {
			self.index += 1;
			return Some(self.point);
		}

		// computing item relatively to point
		let mut item = self.point;

		item += &self.shape[self.index - 1];

		// match self.shape[self.index - 1] {
		// 	Top => item += (0, 1),
		// 	Rgt => item += (1, 0),
		// 	Dwn => item += (0, -1),
		// 	Lft => item += (-1, 0),
		// 	_ => (),
		// }

		self.index += 1;

		// is_rel means next will be computing relatively to prev
		if self.is_rel {
			self.point = item;
		}

		Some(item)
	}
}
