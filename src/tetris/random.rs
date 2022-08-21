use {
	crate::{
		color::{
			Color::{
				self,
				*,
			},
			N_COLORS,
		},
	},
	rand::{
		prelude::*,
		random,
		thread_rng,
	},
};


// generate random u8 different from the previous one
pub fn non_serial_rnd(module: u8) -> u8 {
	static mut PREV: u8 = 0;
	let mut rnd = random::<u8>() % module;

	unsafe {	// danger zone
		while rnd == PREV {
			rnd = random::<u8>() % module;
		}
		PREV = rnd;
	}

	rnd
}


// build sequence of all possible colors
pub fn shuffle_colors() -> [Color; N_COLORS] {
	let mut rng = thread_rng();
	let mut colors = Color::all();
	colors.shuffle(&mut rng);

	colors
}
