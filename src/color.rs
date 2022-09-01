use iced::Color as IcedColor;

pub const N_COLORS: usize = 8;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	Red,
	Grn,
	Blu,
	Wht,
	Ylw,
	Pnk,
	Vin,
	Gry,
	Non,
}


impl Color {
	pub fn all() -> [Color; N_COLORS] {
		[Color::Red, 
		 Color::Grn,
		 Color::Blu,
		 Color::Wht,
		 Color::Ylw,
		 Color::Pnk,
		 Color::Vin,
		 Color::Gry]
	}
	
	pub fn to_rgb(&self) -> IcedColor {
		match self {
			Color::Red => IcedColor::from_rgb8(159,  70,  54),
			Color::Grn => IcedColor::from_rgb8(  7,  87,  91),
			Color::Blu => IcedColor::from_rgb8(120, 165, 163),
			Color::Wht => IcedColor::from_rgb8(241, 220, 201),
			Color::Ylw => IcedColor::from_rgb8(225, 177, 106),
			Color::Pnk => IcedColor::from_rgb8(255, 204, 187),
			Color::Vin => IcedColor::from_rgb8( 30,   0,   0),
			Color::Gry => IcedColor::from_rgb8(254, 155, 151),
			Color::Non => IcedColor::from_rgb8(  0,   0,   0),
		}
	}
}
