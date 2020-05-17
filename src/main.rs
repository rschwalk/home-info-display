use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod networking;

fn main() {
    let command_queue: VecDeque<networking::Commands> = VecDeque::new();
    let queue_mutex = Arc::new(Mutex::new(command_queue));

    let thread_mutex = queue_mutex.clone();
    let handle = thread::spawn(move || {
        networking::listen(thread_mutex);
    });

    let mut running = true;
    while running {
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

        thread::sleep(Duration::from_secs(1));
    }
    handle.join().unwrap();

    println!("Done");
}
