use {
	crate::{
		color::{
			Color,
			N_COLORS,
		},
		tetris::{
			Tetris,
		},
	},
	sdl2::{
		video::{
			Window, WindowContext,
		},
		render::{
			Canvas, Texture, TextureCreator,
		},
		event::{
			Event,
			EventPollIterator,
		},
		rect::Rect,
		keyboard::Keycode,
		VideoSubsystem,
		EventPump,
	},
	std::{
		collections::{
			HashMap,
			hash_map::{
				Entry,
			},
		},
	},
};


const WINDOW_SIZE: u32 = 1000;
const SQUARE_SIZE: u32 = 50;


pub struct Sdl<'a> {
	canvas: Canvas<Window>,
	listener: EventPump,
	squares: HashMap<Color, Texture<'a>>,
}

pub struct Textureer {
	textureer: TextureCreator<WindowContext>
}

impl<'a> Sdl<'a> {
	pub fn init() -> Sdl<'a> {
		// initializing sdl
		let context = sdl2::init()
			.expect("Unable to init SDL");

		// getting video subsystem
		let video = context
			.video()
			.expect("Unable to access video subsystem");

		// creating window
		let window = video
			.window("Tetris", WINDOW_SIZE, WINDOW_SIZE)
			.position_centered()
			.opengl()
			.build()
			.expect("Failed to create window");

		// converting window into canvas
		let mut canvas = window
			.into_canvas()
			.target_texture()
			.present_vsync()
			.build()
			.expect("Failed to create canvas");

		// event handler
		let listener = context.event_pump()
			.expect("Unable to access events");

		let squares = HashMap::with_capacity(N_COLORS);
				
		Sdl {
			canvas,
			listener,
			squares,
		}
	}

	pub fn render(&mut self, tetris: &Tetris) {
		// preparing to re-rendering
		self.canvas.clear();

		// coord related to the tetris map
		for (coord, color) in tetris.map_iter() {
			
		}

		// updating canvas
		self.canvas.present();
	}

	// returns iterator by events to main.rs
	pub fn listen(&mut self) -> EventPollIterator {
		self.listener.poll_iter()
	}

	pub fn textureer(&self) -> TextureCreator<WindowContext> {
		self.canvas.texture_creator()
	}

	pub fn squares(&mut self, textureer: &'a TextureCreator<WindowContext>) {
		for color in Color::all() {
			let square = square_texture(&mut self.canvas,
										textureer,
										color);

			self.squares.insert(color, square);
		}
	}
}

// texture will be stored in sdl.squares and used in rendering
fn square_texture<'a>(canvas: &mut Canvas<Window>,
				  textureer: &'a TextureCreator<WindowContext>,
				  color: Color) -> Texture<'a> {

	let mut square = textureer
		.create_texture_target(None, SQUARE_SIZE, SQUARE_SIZE)
		.expect("Failed to create texture");

	// temporarly converting square_texture into canvas
	canvas.with_texture_canvas(&mut square,
		|texture| {
			texture.set_draw_color(color.to_rgb());
			texture.clear();
		})
		.expect("Failed to color texture");

	square
}
