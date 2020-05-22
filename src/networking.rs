use std::collections::VecDeque;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Commands {
    Quit,
    Invalid,
}

pub fn listen(queue: Arc<Mutex<VecDeque<Commands>>>) {
    let listener = TcpListener::bind("0.0.0.0:62000").unwrap();

    println!("Listening...");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(a) => a,
            _ => unreachable!(),
        };
        let queue = queue.clone();
        let status = handle_connection(stream, queue);
        if !status {
            break;
        }
    }
}

fn handle_connection(mut stream: TcpStream, queue: Arc<Mutex<VecDeque<Commands>>>) -> bool {
    let mut buffer = [0; 512];
    // let len = stream.read(&mut buffer).unwrap();
    // let cmd = String::from_utf8_lossy(&buffer[..len]);
    // println!("{} Byte received.", len);
    // println!("Data received: {}", cmd);

    // true

    match stream.read(&mut buffer) {
        Ok(len) => {
            let cmd = String::from_utf8_lossy(&buffer[..len]);
            let mut commands = queue.lock().unwrap();
            println!("Data received: {}", cmd);
            return if cmd == "terminate\r\n" {
                commands.push_back(Commands::Quit);
                false
            } else {
                commands.push_back(Commands::Invalid);
                true
            };
            // match cmd {
            //     "DONE\r\n" => queue.push_back(Commands::Quit),
            //     _ => queue.push_back(Commands::Invalid),
            // }
        }
        Err(_) => true,
    }
}
