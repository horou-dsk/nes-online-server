use std::thread::{self, JoinHandle};
use chrono::{Local};
use actix::clock::Duration;
use crate::online_server::OnLineServer;
use actix::Addr;

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

pub struct Room {
    frame_buffer: Vec<Vec<u8>>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            frame_buffer: Vec::new(),
        }
    }

    pub fn startup(&mut self, addr: Addr<OnLineServer>) {
        // let mut frame_buffer = &mut self.frame_buffer;
        // thread::spawn(|| {
        //     let mut next_game_tick = Local::now().timestamp_nanos() as f64;
        //     loop {
        //         next_game_tick += MS_PER_UPDATE;
        //         let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
        //         while let Some(v) = frame_buffer.pop() {
        //             // addr.do_send();
        //         }
        //         if sleep_time > 0.0 {
        //             thread::sleep(Duration::from_nanos(sleep_time as u64));
        //         }
        //     }
        // });
    }
}

