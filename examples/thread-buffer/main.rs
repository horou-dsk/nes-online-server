use std::time::Duration;
use std::thread;
use chrono::Local;
use std::thread::JoinHandle;
use actix::{Actor, Context, Addr, AsyncContext, Message, Handler, Running, ActorContext};
use actix_rt::System;

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

pub struct Room {
    frame_buffer: Vec<Vec<u8>>,
    addr: Option<Addr<Room>>,
    stopped: bool,
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
    }
    //
    // fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
    //     let addr = ctx.address();
    //     println!("正在关闭");
    //     addr.do_send(Msg { id: -3 });
    //     Running::Stop
    // }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("已关闭");
    }
}

#[derive(Message)]
#[rtype(isize)]
struct Msg {
    id: i64,
}

impl Handler<Msg> for Room {
    type Result = isize;

    fn handle(&mut self, msg: Msg, ctx: &mut Context<Self>) -> Self::Result {
        if msg.id == -3 {
            ctx.stop();
            // self.stopped = true;
        }
        if msg.id < 0 {
            self.startup(msg.id);
        } else {
            let mut frame_buffer = &mut self.frame_buffer;
            // println!("{}", msg.id);
            while let Some(v) = frame_buffer.pop() {
                // println!("{:?}", v);
                // addr.do_send();
            }
        }
        if self.stopped {
            -1
        } else {
            0
        }
    }
}

impl Room {
    pub fn new() -> Self {
        Self {
            frame_buffer: vec![vec![1, 8, 24]; 5],
            addr: None,
            stopped: false,
        }
    }

    pub fn startup(&mut self, id: i64) {
        if let Some(addr) = self.addr.clone() {
            actix::spawn(async move {
                let mut previous = Local::now().timestamp_millis();
                let mut next_game_tick = Local::now().timestamp_nanos() as f64;
                let mut fps = 0;
                loop {
                    let current = Local::now().timestamp_millis();
                    if current - previous >= 1000 {
                        if id == -2 {
                            println!("fps-- = {}", fps);
                        } else {
                            println!("fps = {}", fps);
                        }
                        fps = 0;
                        previous = current;
                    }
                    fps += 1;
                    next_game_tick += MS_PER_UPDATE;
                    let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
                    let result = addr.send(Msg { id: fps }).await;
                    match result {
                        Ok(i) => {
                            if i < 0 {
                                println!("结束");
                                break;
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err);
                            break;
                        }
                    }
                    if sleep_time > 0.0 {
                        actix::clock::delay_for(Duration::from_nanos(sleep_time as u64)).await;
                        // tokio::time::sleep(Duration::from_nanos(sleep_time as u64)).await;
                        // thread::sleep(if id == -2 {Duration::from_nanos(sleep_time as u64)} else {Duration::from_secs(3)});
                    }
                }
            });
        }
    }
}

fn main() {
    let system = System::new("room");
    let mut room = Room::new().start();
    room.do_send(Msg { id: -1 });
    println!("我他吗服了");
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        room.do_send(Msg { id: -3 });
    });
    // thread::sleep(Duration::from_millis(500));
    // let mut room = Room::new().start();
    // room.do_send(Msg { id: -2 });

    // let mut room = Room::new().start();
    // room.do_send(Msg { id: 0 });
    // let mut room = Room::new().start();
    // room.do_send(Msg { id: 0 });
    // let mut room = Room::new().start();
    // room.do_send(Msg { id: 0 });
    // let mut room = Room::new().start();
    // room.do_send(Msg { id: 0 });
    // room.send().aw

    // join_handle.join();
    // System::current().stop();
    system.run();
    // thread::sleep(Duration::from_secs(8));
}