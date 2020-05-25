extern crate sdl2;

use super::data::MainData;

use sdl2::image::{InitFlag, LoadTexture};
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
    main_data: MainData,
}

enum Align {
    CenterX,
    CenterY,
    Nothing,
}

impl MainDisplay {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let sdl_context = sdl2::init().expect("SDL_Init was not successful!");
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();

        let window = video_subsystem
            .window("Pi Home Display", screen_width, screen_height)
            .position_centered()
            .opengl()
            .build()
            .expect("Window build error!");

        let canvas = window.into_canvas().build().expect("Canvas create error!");
        let texture_creator = canvas.texture_creator();
        let ttf_context = sdl2::ttf::init().expect("TTF init failed!");

        let main_data = MainData::load_data();
        // let timer = sdl_context.timer().expect("Init timer failed");
        // timer.add_timer(1000, move || println!("Hello:") );

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
            main_data,
        }
    }

    pub fn update_weather_data(&mut self) {
        let main_data = MainData::load_data();
        self.main_data = main_data;
        println!("Updeted the weather data.");
    }

    pub fn init(&mut self) {
        self.draw_frame();
        self.draw_labels();
        // self.draw_invalid_temp();
        let current_temp = self.main_data.current_weather.temp as i16;
        self.draw_current_temp(current_temp);
        self.draw_current_weather();
        self.display_calendar();
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

    fn draw_weather_icon(&mut self) {
        let icon_path = format!("assets/img/{}", self.main_data.current_weather.icon);
        let icon_texture = self
            .texture_creator
            .load_texture(icon_path)
            .expect("Failed to load the weather icon!");
        let target = Rect::new(4, 4, 92, 92);
        self.canvas
            .copy(&icon_texture, None, target)
            .expect("Failed to render the current weather icon!");
    }

    fn draw_current_weather(&mut self) {
        self.draw_weather_icon();
        let desc = self.main_data.current_weather.description.clone();
        self.draw_label(
            desc.as_str(),
            16,
            FontStyle::NORMAL,
            104,
            66,
            Align::Nothing,
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

    fn display_calendar(&mut self) {
        let mut pos_y = 118;
        let calendar = self.main_data.cal_data.clone();
        for event in calendar {
            self.draw_label(
                event.as_str(),
                16,
                FontStyle::NORMAL,
                4,
                pos_y,
                Align::Nothing,
            );
            self.canvas.set_draw_color(self.accent_color);
            self.canvas
                .fill_rect(Rect::new(0, pos_y + 22, 400 - 9, 2))
                .expect("Drawing the line failed!");
            pos_y += 24;
        }
    }
}
