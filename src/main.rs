extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::collections::VecDeque;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod networking;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// If we are running on our development machine, then we need to send the terminate message to
// the Tcp socket to end listening.
#[cfg(not(target_arch = "arm"))]
fn send_terminate() {
    match TcpStream::connect("127.0.0.1:62000") {
        Ok(mut stream) => {
            let mut term = b"terminate\r\n";
            stream.write(&term[..]).unwrap();
        }
        Err(e) => println!("Unable to connect to the localhost: {}", e.description()),
    }
}

fn main() {
    let command_queue: VecDeque<networking::Commands> = VecDeque::new();
    let queue_mutex = Arc::new(Mutex::new(command_queue));

    let thread_mutex = queue_mutex.clone();
    let handle = thread::spawn(move || {
        networking::listen(thread_mutex);
    });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let window = video_subsystem
        .window("Pi Home Display", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator = canvas.texture_creator();
    let mut font = ttf_context.load_font("./NotoSans-Regular.ttf", 16).unwrap();

    let surface = font
        .render("Hello rust")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())
        .unwrap();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::RGB(195, 217, 255));
    canvas.clear();
    // canvas.present();

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(64, 64, width as u32, height as u32);

    // let padding = 64;
    // let target = get_centered_rect(width, height, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);

    canvas.copy(&texture, None, Some(target)).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,

                Event::Window {
                    win_event: WindowEvent::Close,
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    send_terminate();
                    running = false
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.copy(&texture, None, Some(target)).unwrap();
        canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        {
            let mut commands = queue_mutex.lock().unwrap();
            if !commands.is_empty() {
                let cmd = commands.pop_front().unwrap();
                println!("Command is: {:#?}", cmd);
                match cmd {
                    networking::Commands::Quit => running = false,
                    networking::Commands::Invalid => {}
                }
            }
        }

        thread::sleep(Duration::from_secs(1 / 2));
    }

    handle.join().unwrap();

    println!("Done");
}
