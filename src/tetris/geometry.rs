use {
	std::{
		ops::{
			Add,
			AddAssign,
			Sub,
			SubAssign,
			Div,
			Neg,
		},
		cmp::{
			Eq,
			Ord,
		},
	},
	num::{
		Signed,
		abs,
	},
};


// since it's just two num's it may represent point or vector
#[derive(Debug, Clone, Copy)]
pub struct Coord<T> {
	pub x: T,
	pub y: T,
}

// tuple represents motion vector
impl<T> Add<(T, T)> for Coord<T>
where T: Add<Output = T> + Copy
{
	type Output = Coord<T>;

	fn add(self, other: (T, T)) -> Self::Output {
		Coord {
			x: self.x + other.0,
			y: self.y + other.1,
		}
	}
}

impl<T> AddAssign<(T, T)> for Coord<T>
where T: Add<Output = T> + Copy
{
	fn add_assign(&mut self, other: (T, T)) {
		*self = Self {
			x: self.x + other.0,
			y: self.y + other.1,
		};
	}
}

// &Coord represents motion vector
impl<T> AddAssign<&Coord<T>> for Coord<T>
where T: Add<Output = T> + Copy
{
	fn add_assign(&mut self, other: &Coord<T>) {
		*self = Coord {
			x: self.x + other.x,
			y: self.y + other.y,
		};
	}
}

impl<T> AddAssign<&Direction> for Coord<T>
where T: Add<Output = T> + From<i8> + Copy
{
	fn add_assign(&mut self, dir: &Direction) {
		let x_addition = match dir {
			Direction::Rgt => T::from(1),
			Direction::Lft => T::from(-1),
			_ => T::from(0),
		};
		let y_addition = match dir {
			Direction::Top => T::from(1),
			Direction::Dwn => T::from(-1),
			_ => T::from(0),
		};

		*self = Coord {
			x: self.x + x_addition,
			y: self.y + y_addition,
		};
	}
}

impl<T> SubAssign<&Coord<T>> for Coord<T>
where T: Sub<Output = T> + Copy
{
	fn sub_assign(&mut self, other: &Coord<T>) {
		*self = Coord {
			x: self.x - other.x,
			y: self.y - other.y,
		};
	}
}

impl<T> From<(T, T)> for Coord<T> {
	fn from(point: (T, T)) -> Self {
		Self {
			x: point.0,
			y: point.1,
		}
	}
}


// includes both ends
#[derive(Debug)]
pub struct Segment<T> {
	pub lhe: T,		// left-hand end
	pub rhe: T,		// right-hand end
}

// T represents shifting on the axis
impl<T> Add<T> for &Segment<T>
where T: Add<Output = T> + Copy
{
	type Output = Segment<T>;

	fn add(self, motion: T) -> Self::Output {
		Segment {
			lhe: self.lhe + motion,
			rhe: self.rhe + motion,
		}
	}
}

impl<T> From<(T, T)> for Segment<T> {
	fn from(segment: (T, T)) -> Self {
		Self {
			lhe: segment.0,
			rhe: segment.1,
		}
	}
}

impl<T> Segment<T>
where T: Add<Output = T> + Sub<Output = T> + Ord + From<i8> + Copy
{
	// arranges the ends in ascending order
	pub fn ascending_order(&mut self) {
		if self.lhe > self.rhe {
			*self = Self {
				lhe: self.rhe,
				rhe: self.lhe,
			};
		}
	}

	// make sure the segment is arranged for the following fn's 
	pub fn include_point(&self, point: T) -> bool {
		self.lhe <= point && point <= self.rhe
	}

	pub fn include_segment(&self, other: Segment<T>) -> bool {
		self.lhe <= other.lhe && other.rhe <= self.rhe
	}

	pub fn len(&self) -> T {
		self.rhe - self.lhe + T::from(1)
	}

	// transforms segment so that it starts to have
	// the taken point as one of the ends if it wasn't included
	pub fn extend(&mut self, point: T) {
		if point < self.lhe {
			self.lhe = point;
		}
		else if self.rhe < point {
			self.rhe = point;
		}
	}

	// provides iterating by int's on the segment
	pub fn iter(&self) -> SegmentIter<T> {
		SegmentIter {
			inited: true,
			curr: T::from(0),
			segment: self,
		}
	}
}


// iterating by int's on the segment in the following order
// one int to the right from zero, then one to the left, etc
pub struct SegmentIter<'a, T> {
	// true if iter was just created or reseted
	inited: bool,
	// when next() called it store prev returned int 
	curr: T,
	// segment itself contains int's by which iteration is performed
	segment: &'a Segment<T>,
}

impl<T> Iterator for SegmentIter<'_, T>
where T: Add<Output = T> + AddAssign + Sub<Output = T> +
		 Neg<Output = T> + Eq + Ord + From<i8> + Copy
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let zero = T::from(0);

		// finds the initial int
		if self.inited {
			self.inited = false;
			self.curr =
				if self.segment.include_point(zero) {
					zero
				} else if self.segment.lhe > zero {
					self.segment.lhe
				} else {
					self.segment.rhe
				};

			return Some(self.curr);
		}

		// first two iterations looks for next int from
		// both sides from zero, third shows iterator ends
		for i in 0..3 {
			self.curr = -self.curr;
			if self.curr >= zero {
				self.curr += T::from(1);
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

// impl<T> SegmentIter<'_, T> {
// 	// provides possibility of restarting iteration
// 	// from the beginning without recreating iterator
// 	fn reset(&mut self) {
// 		self.inited = true;
// 	}
// }




// stored as two Segments, includes border points
#[derive(Debug)]
pub struct Rectangle<T> {
	pub x_axis: Segment<T>,		// bounds on the axis of x
	pub y_axis: Segment<T>,		// bounds on the axis of y
}

// &Coord represents motion vector
impl<T> AddAssign<&Coord<T>> for Rectangle<T>
where T: Add<Output = T> + Copy
{
	fn add_assign(&mut self, motion: &Coord<T>) {
		*self = Self {
			x_axis: &self.x_axis + motion.x,
			y_axis: &self.y_axis + motion.y,
		};
	}
}

// such as two Segment's define rectangle
// rectangle can be defined by two Coord's
impl<T> From<(&Coord<T>, &Coord<T>)> for Rectangle<T>
where T: Add<Output = T> + Sub<Output = T> + Ord + From<i8> + Copy
{
	fn from(points: (&Coord<T>, &Coord<T>)) -> Rectangle<T> {
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

impl<T> Rectangle<T>
where T: Add<Output = T> + Sub<Output = T> + Div<T, Output = T> +
		 Neg<Output = T> + From<i8> + Ord + Signed + Copy
{
	pub fn include_point(&self, x: T, y: T) -> bool {
		self.x_axis.include_point(x) && self.y_axis.include_point(y)
	}

	// transforms rectangle so that it starts
	// to include the taken point, if it didn't
	pub fn extend(&mut self, point: &Coord<T>) {
		self.x_axis.extend(point.x);
		self.y_axis.extend(point.y);
	}

	// moves rectangle so that self and other form letter T upside down
	// returns motion vector represented as Coord
	pub fn move_to(&mut self, other: &Rectangle<T>) -> Coord<T> {
		let self_width = self.x_axis.len();
		let other_width = other.x_axis.len();

		// how far away from the other left side self will be placed
		let mut offset_x = abs(self_width - other_width) / T::from(2);

		// it has the same sign as the direction in which self left
		// border will be placed relatively to the other left border
		if other_width < self_width {
			offset_x = -offset_x;
		}

		// a little magic computes the motion vector 
		let motion = Coord {
			x: other.x_axis.lhe + offset_x - self.x_axis.lhe,
			y: other.y_axis.lhe - self.y_axis.lhe,
		};
		*self += &motion;

		motion
	}

	// provides iterating by all inner and border points
	pub fn iter(&self) -> RectangleIter<T> {
		RectangleIter {
			inited: true,
			curr_y: T::from(0),
			y_iter: self.y_axis.iter(),
			x_iter: self.x_axis.iter(),
		}
	}
}


// iterating by all int points the rectangle includes
// in following order: first by y iter, then by x iter
pub struct RectangleIter<'a, T> {
	inited: bool,
	curr_y: T,
	y_iter: SegmentIter<'a, T>,
	x_iter: SegmentIter<'a, T>,
}

impl<T> Iterator for RectangleIter<'_, T>
where T: Add<Output = T> + AddAssign + Sub<Output = T> +
		 Neg<Output = T> + Eq + Ord + From<i8> + Copy
{
	type Item = Coord<T>;

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
				return Some(Coord::from((curr_x, self.curr_y)));
			} else {
				if let Some(curr_y) = self.y_iter.next() {
					self.curr_y = curr_y;
					self.x_iter.inited = true;
				} else {
					return None;
				}
			}
		}

		None
	}
}


// means shifting by one unit of measurement
// is used by tetrimino shape and shifting events
#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Into<Coord<i8>> for Direction {
	fn into(self) -> Coord<i8> {
		match self {
			Direction::Top => Coord { x: 0, y: 1 },
			Direction::Rgt => Coord { x: 1, y: 0 },
			Direction::Dwn => Coord { x: 0, y: -1 },
			Direction::Lft => Coord { x: -1, y: 0 },
		}
	}
}
