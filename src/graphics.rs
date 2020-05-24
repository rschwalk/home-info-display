extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, TextureQuery, WindowCanvas};
use sdl2::ttf::{FontStyle, Sdl2TtfContext};
use sdl2::video::WindowContext;
use sdl2::Sdl;

pub struct MainDisplay {
    screen_width: u32,
    screen_height: u32,
    pub sdl_context: Sdl,
    pub canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
    accent_color: Color,
    bg_color: Color,
    fg_color: Color,
}

enum Align {
    CenterX,
    CenterY,
    Nothing,
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
        let texture_creator = canvas.texture_creator();
        let ttf_context = sdl2::ttf::init().expect("TTF init failed!");

        Self {
            screen_width,
            screen_height,
            sdl_context,
            canvas,
            texture_creator,
            ttf_context,
            accent_color: Color::RGB(235, 110, 75),
            bg_color: Color::RGB(72, 72, 74),
            fg_color: Color::RGB(253, 253, 253),
        }
    }

    pub fn init(&mut self) {
        self.draw_frame();
        self.draw_labels();
        self.draw_invalid_temp();
    }

    fn clear_with_bg(&mut self) {
        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();
    }

    fn draw_frame(&mut self) {
        let rects = [
            // vertical
            Rect::new(391, 100, 18, self.screen_height),
            // horizontal
            Rect::new(0, 100, self.screen_width, 18),
            // main temp
            Rect::new(340, 0, 120, 100),
        ];

        self.clear_with_bg();
        self.canvas.set_draw_color(self.accent_color);
        self.canvas
            .fill_rects(&rects)
            .expect("Not able to draw the rectangles!");
    }

    pub fn draw_labels(&mut self) {
        // label CALENDAR
        self.draw_label("CALENDER", 16, FontStyle::BOLD, 200 - 9, 97, Align::CenterX);

        // label To Do
        self.draw_label("TODO", 16, FontStyle::BOLD, 600 - 9, 97, Align::CenterX);

        // label °C
        self.draw_label("°C", 16, FontStyle::BOLD, 340 + 92, 30, Align::Nothing);
    }

    pub fn draw_current_temp(&mut self, value: i16) {
        let label_pos_x = (340 + 4) as i32;
        let label_pos_y = 50;
        self.draw_label(
            value.to_string().as_str(),
            65,
            FontStyle::BOLD,
            label_pos_x,
            label_pos_y,
            Align::CenterY,
        );
    }

    fn draw_invalid_temp(&mut self) {
        let label_pos_x = (340 + 4) as i32;
        let label_pos_y = 50;
        self.draw_label(
            "--",
            65,
            FontStyle::BOLD,
            label_pos_x,
            label_pos_y,
            Align::CenterY,
        );
    }

    fn draw_label(
        &mut self,
        txt: &str,
        font_size: u16,
        font_style: FontStyle,
        pos_x: i32,
        pos_y: i32,
        align: Align,
    ) {
        let mut font = self
            .ttf_context
            .load_font("assets/NotoSans-Regular.ttf", font_size)
            .expect("Font loading failed!");
        font.set_style(font_style);

        let surface = font
            .render(txt)
            .blended(self.fg_color)
            .expect("Font rendering failed!");
        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .expect("Creating the font texture failed!");

        let TextureQuery { width, height, .. } = texture.query();

        let pos = match align {
            Align::CenterX => (pos_x - width as i32 / 2, pos_y),
            Align::CenterY => (pos_x, pos_y - height as i32 / 2),
            Align::Nothing => (pos_x, pos_y),
        };
        let (center_pos_x, center_pos_y) = pos;
        let target = Rect::new(center_pos_x, center_pos_y, width, height);
        self.canvas
            .copy(&texture, None, target)
            .expect("Rendering the calendar label failed!");
    }
}
