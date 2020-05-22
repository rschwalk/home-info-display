extern crate sdl2;

use sdl2::Sdl;

pub struct MainDisplay {
    screen_width: u32,
    screen_height: u32,
    pub context: Sdl,
    pub canvas: sdl2::render::WindowCanvas,
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

        let mut canvas = window.into_canvas().build().expect("Canvas create error!");

        Self {
            screen_width,
            screen_height,
            context: sdl_context,
            canvas,
        }
    }
}
