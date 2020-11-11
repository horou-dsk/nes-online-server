use std::time::Duration;
use std::thread;
use chrono::Local;
use std::thread::JoinHandle;

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

pub struct Room {
    frame_buffer: Vec<Vec<u8>>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            frame_buffer: vec![vec![1, 8, 24]; 5],
        }
    }

    pub fn startup(&mut self) {
        thread::spawn(move || {
            let mut frame_buffer = &mut self.frame_buffer;
            let mut next_game_tick = Local::now().timestamp_nanos() as f64;
            loop {
                next_game_tick += MS_PER_UPDATE;
                let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
                while let Some(v) = frame_buffer.pop() {
                    println!("{:?}", v);
                    // addr.do_send();
                }
                if sleep_time > 0.0 {
                    thread::sleep(Duration::from_nanos(sleep_time as u64));
                }
            }
        });
    }
}

fn main() {
    let mut room = Room::new();
    let join_handle = thread::spawn(move || {
        room.startup()
    });
    join_handle.join();
}