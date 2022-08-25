use {
	sdl2::pixels::Color as SdlColor,
	std::hash::Hash,
};


pub const N_COLORS: usize = 8;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
	
	pub fn to_rgb(&self) -> SdlColor {
		match self {
			Color::Red => SdlColor::RGB(159,  70,  54),
			Color::Grn => SdlColor::RGB(  7,  87,  91),
			Color::Blu => SdlColor::RGB(120, 165, 163),
			Color::Wht => SdlColor::RGB(241, 220, 201),
			Color::Ylw => SdlColor::RGB(225, 177, 106),
			Color::Pnk => SdlColor::RGB(255, 204, 187),
			Color::Vin => SdlColor::RGB( 30,   0,   0),
			Color::Gry => SdlColor::RGB(254, 155, 151),
			Color::Non => SdlColor::RGB(  0,   0,   0),
		}
	}
}
