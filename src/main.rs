extern crate sdl2;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use std::collections::VecDeque;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod graphics;
mod networking;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// If we are running on our development machine, then we need to send the
// terminate message to the Tcp socket to end listening.
#[cfg(not(target_arch = "arm"))]
fn send_terminate() {
    match TcpStream::connect("127.0.0.1:62000") {
        Ok(mut stream) => {
            let term = b"terminate\r\n";
            stream.write(&term[..]).unwrap();
        }
        Err(e) => println!("Unable to connect to the localhost: {}", e.to_string()),
    }
}

fn main() {
    let command_queue: VecDeque<networking::Commands> = VecDeque::new();
    let queue_mutex = Arc::new(Mutex::new(command_queue));

    let thread_mutex = queue_mutex.clone();
    let handle = thread::spawn(move || {
        networking::listen(thread_mutex);
    });

    let mut display = graphics::MainDisplay::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // canvas.set_draw_color(Color::RGB(195, 217, 255));
    // canvas.clear();
    // canvas.present();

    // let TextureQuery { width, height, .. } = texture.query();

    // let target = Rect::new(64, 64, width as u32, height as u32);

    // canvas.copy(&texture, None, Some(target)).unwrap();
    // canvas.present();

    let mut event_pump = display.sdl_context.event_pump().unwrap();

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

        // canvas.clear();
        // display.canvas.clear();
        display.draw_frame();
        display.draw_labels();
        display.draw_current_temp(25);
        display.canvas.present();
        // canvas.copy(&texture, None, Some(target)).unwrap();
        // canvas.present();

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

        thread::sleep(Duration::from_secs(1 / 24));
    }

    handle.join().unwrap();

    println!("Done");
}
