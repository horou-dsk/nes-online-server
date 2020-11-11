use chrono::{Local};
use std::thread;
use actix::clock::Duration;

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

fn main() {
    let mut previous = Local::now().timestamp_millis();
    let mut lag = 0f64;
    let mut fps = 0;
    let mut p = Local::now().timestamp_millis();
    let mut next_game_tick = Local::now().timestamp_nanos() as f64;
    loop {
        let current = Local::now().timestamp_millis();
        if current - p >= 1000 {
            println!("fps = {}, lag = {}", fps, lag);
            fps = 0;
            p = current;
        }
        next_game_tick += MS_PER_UPDATE;
        let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
        fps += 1;
        // previous = current;
        // lag += elapsed as f64;
        // while lag >= MS_PER_UPDATE {
        //     lag -= MS_PER_UPDATE;
        //     fps += 1;
        // }
        // let elapsed = current - previous;
        if sleep_time > 0f64 {
            // println!("{}", sleep_time);
            thread::sleep(Duration::from_nanos(sleep_time as u64))
        }
    }
}