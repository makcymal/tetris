use std::{
	ops::{
		Add,
		AddAssign,
		SubAssign,
	},
};


#[derive(Debug, Clone, Copy)]
// since it's just two num's it may represent coordinate or vector
pub struct Point {
	pub x: i8,
	pub y: i8,
}

// tuple represents motion vector
impl Add<(i8, i8)> for Point {
	type Output = Point;

	fn add(self, other: (i8, i8)) -> Point {
		Point {
			x: self.x + other.0,
			y: self.y + other.1,
		}
	}
}

impl AddAssign<(i8, i8)> for Point {
	fn add_assign(&mut self, other: (i8, i8)) {
		*self = Self {
			x: self.x + other.0,
			y: self.y + other.1,
		};
	}
}

// &Point represents motion vector
impl AddAssign<&Point> for Point {
	fn add_assign(&mut self, other: &Point) {
		*self = Point {
			x: self.x + other.x,
			y: self.y + other.y,
		};
	}
}

impl SubAssign<&Point> for Point {
	fn sub_assign(&mut self, other: &Point) {
		*self = Point {
			x: self.x - other.x,
			y: self.y - other.y,
		};
	}
}

// (x, y) : (i8, i8)
impl From<(i8, i8)> for Point {
	fn from(point: (i8, i8)) -> Point {
		Point {
			x: point.0,
			y: point.1,
		}
	}
}

impl From<Direction> for Point {
	fn from(dir: Direction) -> Point {
		match dir {
			Top => Point::from((0, 1)),
			Rgt => Point::from((1, 0)),
			Dwn => Point::from((0, -1)),
			Lft => Point::from((-1, 0)),
		}
	}
}


#[derive(Debug)]
// includes both ends
pub struct Segment {
	pub lb: i8,		// left border
	pub rb: i8,		// right border
}

impl Segment {
	// provides iterating by int's on the segment
	pub fn iter(&self) -> SegmentIter {
		SegmentIter {
			inited: true,
			curr: 0,
			segment: self,
		}
	}

	// arranges the ends in ascending order
	pub fn ascending_order(&mut self) {
		if self.lb > self.rb {
			*self = Segment {
				lb: self.rb,
				rb: self.lb,
			};
		}
	}

	// make sure the segment is arranged for the following fn's 
	pub fn include_point(&self, point: i8) -> bool {
		self.lb <= point && point <= self.rb
	}

	pub fn include_segment(&self, other: Segment) -> bool {
		self.lb <= other.lb && other.rb <= self.rb
	}

	pub fn len(&self) -> i8 {
		self.rb - self.lb + 1
	}

	// transforms segment so that it starts to have
	// the taken point as one of the ends if it wasn't included
	pub fn extend(&mut self, point: i8) {
		if point < self.lb {
			self.lb = point;
		}
		else if self.rb < point {
			self.rb = point;
		}
	}
}

// i8 represents shifting on the axis
impl Add<i8> for &Segment {
	type Output = Segment;

	fn add(self, motion: i8) -> Segment {
		Segment {
			lb: self.lb + motion,
			rb: self.rb + motion,
		}
	}
}

// (lb, rb) : (i8, i8)
impl From<(i8, i8)> for Segment {
	fn from(segment: (i8, i8)) -> Segment {
		Segment {
			lb: segment.0,
			rb: segment.1,
		}
	}
}


// iterating by int's on the segment in the following order
// one int to the right from zero, then one to the left, etc
pub struct SegmentIter<'a> {
	// true if iter was just created or reseted
	inited: bool,
	// when next() called it store prev returned int 
	curr: i8,
	// segment itself contains int's by which iteration is performed
	segment: &'a Segment,
}

impl SegmentIter<'_> {
	// provides possibility of restarting iteration
	// from the beginning without recreating iterator
	fn reset(&mut self) {
		self.inited = true;
	}
}

impl Iterator for SegmentIter<'_> {
	type Item = i8;

	fn next(&mut self) -> Option<Self::Item> {
		// finds the initial int
		if self.inited {
			self.inited = false;
			self.curr =
				if self.segment.include_point(0) {
					0
				} else if self.segment.lb > 0 {
					self.segment.lb
				} else {
					self.segment.rb
				};

			return Some(self.curr);
		}

		// first two iterations looks for next int from
		// both sides from zero, third shows iterator ends
		for i in 0..3 {
			self.curr *= -1;
			if self.curr >= 0 {
				self.curr += 1;
			}

			if self.segment.include_point(self.curr) {
				break;
			}

			if i == 2 {
				return None;
			}
		}

		Some(self.curr)
	}
}


#[derive(Debug)]
// stored as two Segments, includes border points
pub struct Rectangle {
	pub x_axis: Segment,		// bounds on the axis of x
	pub y_axis: Segment,		// bounds on the axis of y
}

impl Rectangle {
	// provides iterating by all inner and border points
	pub fn iter(&self) -> RectangleIter {
		RectangleIter {
			inited: true,
			curr_y: 0,
			y_iter: self.y_axis.iter(),
			x_iter: self.x_axis.iter(),
		}
	}

	pub fn include_point(&self, x: i8, y: i8) -> bool {
		self.x_axis.include_point(x) && self.y_axis.include_point(y)
	}

	// transforms rectangle so that it starts
	// to include the taken point, if it didn't
	pub fn extend(&mut self, point: &Point) {
		self.x_axis.extend(point.x);
		self.y_axis.extend(point.y);
	}

	// moves rectangle so that self and other form letter T upside down
	// returns motion vector represented as Point
	pub fn move_to(&mut self, other: &Rectangle) -> Point {
		let self_x_len = self.x_axis.len();
		let other_x_len = other.x_axis.len();

		// how far away from the other left side self will be placed
		let mut offset_x = (self_x_len - other_x_len).abs() / 2;

		// it has the same sign as the direction in which self left
		// border will be placed relatively to the other left border
		if other_x_len < self_x_len {
			offset_x *= -1;
		}

		// a little magic computes the motion vector 
		let motion = Point {
			x: other.x_axis.lb + offset_x - self.x_axis.lb,
			y: other.y_axis.lb - self.y_axis.lb,
		};
		*self += &motion;

		motion
	}
}

// &Point represents motion vector
impl AddAssign<&Point> for Rectangle {
	fn add_assign(&mut self, motion: &Point) {
		*self = Self {
			x_axis: &self.x_axis + motion.x,
			y_axis: &self.y_axis + motion.y,
		};
	}
}

// such as two Segment's define rectangle
// rectangle can be defined by two Point's
impl From<(&Point, &Point)> for Rectangle {
	fn from(points: (&Point, &Point)) -> Rectangle {
		let mut x_axis = Segment::from((points.0.x, points.1.x));
		x_axis.ascending_order();

		let mut y_axis = Segment::from((points.0.y, points.1.y));
		y_axis.ascending_order();

		Rectangle {
			x_axis,
			y_axis,
		}
	}
}


// iterating by all int points the rectangle includes
// in following order: first by y iter, then by x iter
pub struct RectangleIter<'a> {
	inited: bool,
	curr_y: i8,
	y_iter: SegmentIter<'a>,
	x_iter: SegmentIter<'a>,
}

impl Iterator for RectangleIter<'_> {
	type Item = Point;

	fn next(&mut self) -> Option<Self::Item> {
		// inited only needed for computing initial y
		if self.inited {
			self.inited = false;
			if let Some(curr_y) = self.y_iter.next() {
				self.curr_y = curr_y;
			} else {
				return None;
			}
		}

		// if x iter returns None it's time to take next from y iter
		// after that it's time to try to take next from x iter
		for _ in 0..2 {
			if let Some(curr_x) = self.x_iter.next() {
				return Some(Point::from((curr_x, self.curr_y)));
			} else {
				if let Some(curr_y) = self.y_iter.next() {
					self.curr_y = curr_y;
					self.x_iter.reset();
				} else {
					return None;
				}
			}
		}

		None
	}
}


#[derive(Debug)]
// means shifting by one unit of measurement
// is used by tetrimino shape and shifting events
pub enum Direction {
	Top,
	Rgt,
	Dwn,
	Lft,
}

impl Direction {
	pub fn rotate(&mut self, clockwise: bool) {
		match clockwise {
			true => self.clockwise(),
			false => self.counterclockwise(),
		};
	}

	fn clockwise(&mut self) {
		*self = match self {
			Direction::Top => Direction::Rgt,
			Direction::Rgt => Direction::Dwn,
			Direction::Dwn => Direction::Lft,
			Direction::Lft => Direction::Top,
		};
	}

	fn counterclockwise(&mut self) {
		*self = match self {
			Direction::Top => Direction::Lft,
			Direction::Lft => Direction::Dwn,
			Direction::Dwn => Direction::Rgt,
			Direction::Rgt => Direction::Top,
		}
	}
}
