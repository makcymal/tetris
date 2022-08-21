use {
    crate::{
        color::Color,
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
};


pub struct Sdl {
    canvas: Canvas<Window>,
    textureer: TextureCreator<WindowContext>,
    listener: EventPump,
}

impl Sdl {
    pub fn init() -> Sdl {
        // initializing sdl
        let context = sdl2::init()
            .expect("Unable to init SDL");

        // getting video subsystem
        let video = context
            .video()
            .expect("Unable to access video subsystem");

        // creating window
        let window = video
            .window("Tetris", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window");

        // converting window into canvas
        let canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .expect("Failed to create canvas");

        // this thing can create textures
        let textureer = canvas.texture_creator();

        // event handler
        let listener = context.event_pump()
            .expect("Unable to access events");

        Sdl {
            canvas,
            textureer,
            listener,
        }
    }

    pub fn listen(&mut self) -> EventPollIterator {
        self.listener.poll_iter()
    }

    pub fn set_color(&mut self, color: Color) {
        // setting default color
        self.canvas.set_draw_color(color.to_rgb());
    }

    pub fn clear_canvas(&mut self) {
        // preparing for drawing
        self.canvas.clear();
    }

    pub fn update_canvas(&mut self) {
        self.canvas.present();
    }

    pub fn create_rect(&mut self,
                       color: Color,
                       size: u32) -> Option<Texture> {

        // trying to create rect texture
        if let Ok(mut square_texture) =
        self.textureer.create_texture_target(None, size, size) {

            // this temporarly converts square_texture into canvas
            self.canvas.with_texture_canvas(&mut square_texture,
                                            |texture| {
                                                texture.set_draw_color(color.to_rgb());
                                                texture.clear();
                                            })
                .expect("Failed to color texture");

            Some(square_texture)
        } else {
            None
        }
    }
}
