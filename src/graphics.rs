extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;

pub struct MainDisplay {
    // screen_width: u32,
    // screen_height: u32,
    pub context: Sdl,
    pub canvas: sdl2::render::WindowCanvas,
    accent_color: sdl2::pixels::Color,
    bg_color: sdl2::pixels::Color,
}

impl MainDisplay {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let sdl_context = sdl2::init().expect("SDL_Init was not sucsessfull!");
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Pi Home Display", screen_width, screen_height)
            .position_centered()
            .opengl()
            .build()
            .expect("Window build error!");

        let canvas = window.into_canvas().build().expect("Canvas create error!");

        Self {
            // screen_width,
            // screen_height,
            context: sdl_context,
            canvas,
            accent_color: Color::RGB(235, 110, 75),
            bg_color: Color::RGB(72, 72, 74),
        }
    }

    fn clear_with_bg(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
    }

    pub fn draw_frame(&mut self) {
        let rects = [
            Rect::new(392, 100, 16, 600),
            Rect::new(0, 100, 800, 16),
            Rect::new(340, 0, 120, 100),
        ];

        self.clear_with_bg();
        self.canvas.set_draw_color(self.accent_color);
        self.canvas
            .fill_rects(&rects)
            .expect("Not able to draw the rectangles!");
    }
}
