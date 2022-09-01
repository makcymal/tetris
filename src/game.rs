use {
	iced::{
		canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
		canvas::event::{self, Event},
		keyboard::{
			KeyCode, Modifiers,
			Event::KeyPressed,
		}, 
		executor, time, window, Application, Color, Command, Container,
		Element, Length, Point, Rectangle, Settings, Size, Subscription,
	},
	crate::{
		color::Color::*,
		tetris::Tetris,
	},
};


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Msg {
	Proceed,
	ShiftRgt,
	ShiftLft,
	Clockwise,
	Counterclockwise,
	Exit,
}


pub struct Game {
	tetris: Tetris,
	cache: Cache,
}


impl Application for Game {
	type Executor = executor::Default;
	type Message = Msg;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Msg>) {
		(
			Self {
				tetris: Tetris::new(),
				cache: Default::default(),
			},
			Command::none(),
		)
	}

	fn mode(&self) -> window::Mode {
		window::Mode::Fullscreen
	}

	fn title(&self) -> String {
		String::from("Tetris")
	}

	fn update(&mut self, msg: Msg) -> Command<Msg> {
		if msg == Msg::Proceed {
			self.cache.clear();
			self.tetris.proceed();
		}
        Command::none()
	}

	fn subscription(&self) -> Subscription<Msg> {
        time::every(self.tetris.level_time()).map(|_| {
            Msg::Proceed
        })
    }

	fn view(&mut self) -> Element<Msg> {
		let canvas = Canvas::new(self).width(Length::Fill).height(Length::Fill);

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
	}
}

impl<Message> canvas::Program<Message> for Game {
	fn update(&mut self,
              ev: Event,
              _: Rectangle, _: Cursor, ) -> (event::Status, Option<Message>) {

		if let Event::Keyboard(KeyPressed{ key_code, modifiers }) = ev {
			match (key_code, modifiers.control()) {
				(KeyCode::Right, false) => {
					self.cache.clear();
					self.tetris.react_to(Msg::ShiftRgt);
				}
				(KeyCode::Left, false) => {
					self.cache.clear();
					self.tetris.react_to(Msg::ShiftLft);
				}
				(KeyCode::Right, true) => {
					self.cache.clear();
					self.tetris.react_to(Msg::Clockwise);
				}
				(KeyCode::Left, true) => {
					self.cache.clear();
					self.tetris.react_to(Msg::Counterclockwise);
				}
				(KeyCode::Down, _) => {
					self.cache.clear();
					self.tetris.proceed();
				}
				(KeyCode::Escape, _) => std::process::exit(0),
				_ => (),
			};
		}

		(event::Status::Ignored, None)
	}

	fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
		let side = bounds.size().height * 0.8 / 16.0;
		let size = Size { width: side, height: side };

		let cache = self.cache.draw(bounds.size(), |frame| {
			let map = self.tetris.map_iter();
			for (coord, color) in map {
				let point = Point::new(side * (coord.x as f32),
									   side * (16.0 - coord.y as f32));
            	let background = Path::rectangle(point, size);
            	frame.fill(&background, color.to_rgb());
			}
		});

		vec!(cache)
	}
}
