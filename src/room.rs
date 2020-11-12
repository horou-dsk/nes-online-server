use std::time::Duration;
use std::thread;
use chrono::Local;
use std::thread::JoinHandle;
use actix::{Actor, Context, Addr, AsyncContext, Message, Handler, Running, ActorContext};
use actix_rt::System;
use crate::online_server::{OnLineServer, ClientMessage};
use std::collections::HashSet;
use crate::proto::SendParcel;
use log::{info};

const MS_PER_UPDATE: f64 = 100000000.0 / 6.0;

pub struct Room {
    frame_buffer: Vec<Vec<u8>>,
    addr: Option<Addr<Room>>,
    stopped: bool,
    online_addr: Addr<OnLineServer>,
    name: String,
    //Todo: 暂时没用这个
    sessions: HashSet<usize>,
    running: bool,
}

impl Room {
    pub fn new(name: String, addr: Addr<OnLineServer>) -> Self {
        Self {
            frame_buffer: Vec::new(),
            name,
            stopped: false,
            addr: None,
            online_addr: addr,
            sessions: HashSet::new(),
            running: false,
        }
    }

    pub fn startup(&mut self) {
        if self.running {
            return;
        }
        if let Some(addr) = self.addr.clone() {
            self.running = true;
            info!("游戏开始");
            actix::spawn(async move {
                let mut next_game_tick = Local::now().timestamp_nanos() as f64;
                loop {
                    next_game_tick += MS_PER_UPDATE;
                    let sleep_time = next_game_tick - Local::now().timestamp_nanos() as f64;
                    let result = addr.send(Frame).await;
                    match result {
                        Ok(i) => { if i < 0 {
                            info!("游戏结束");
                            break;
                        } }
                        Err(err) => { break; }
                    }
                    if sleep_time > 0.0 {
                        actix::clock::delay_for(Duration::from_nanos(sleep_time as u64)).await;
                    }
                }
            });
        }
    }
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("已关闭");
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomStart;

impl Handler<RoomStart> for Room {
    type Result = ();

    fn handle(&mut self, _: RoomStart, _: &mut Context<Self>) -> Self::Result {
        self.startup();
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomStop;

impl Handler<RoomStop> for Room {
    type Result = ();

    fn handle(&mut self, _: RoomStop, ctx: &mut Context<Self>) -> Self::Result {
        self.running = false;
        // ctx.stop();
    }
}

#[derive(Message)]
#[rtype(isize)]
struct Frame;

impl Handler<Frame> for Room {
    type Result = isize;

    fn handle(&mut self, msg: Frame, _: &mut Context<Self>) -> Self::Result {
        self.online_addr.do_send(ClientMessage {
            id: 0,
            room: self.name.clone(),
            msg: serde_json::to_string(&SendParcel::SendKeyboard(self.frame_buffer.clone())).unwrap(),
        });
        self.frame_buffer.clear();
        if self.running { 1 } else { -1 }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PushBuffer(pub Vec<u8>);

impl Handler<PushBuffer> for Room {
    type Result = ();

    fn handle(&mut self, msg: PushBuffer, _: &mut Context<Self>) -> Self::Result {
        self.frame_buffer.push(msg.0);
    }
}
