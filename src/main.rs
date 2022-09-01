#![allow(warnings)]

mod color;
mod game;
mod tetris;

use {
	color::Color,
	game::Game,
	tetris::Tetris,
	iced::{window, Application, Settings},
};


fn main() -> iced::Result {
	Game::run(Settings::default())

	// let mut tetris = Tetris::new();
	// tetris.proceed();
	// for tile in tetris.map_iter() {
	// 	println!("{:?}", tile);
	// }
}
