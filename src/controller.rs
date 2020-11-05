/*use actix::Addr;
use crate::online_server as server;
use serde_json::{Value};
use log::{info};
use crate::online_server::ClientMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GameReady {
    mid: u8,
}

impl GameReady {
    fn new() -> Self {
        Self { mid: 1 }
    }
}

pub fn on_message(session_id: usize, addr: &Addr<server::OnLineServer>, msg: String) {
    let v: Value = serde_json::from_str(msg.as_str()).unwrap();
    match v["mid"].as_u64() {
        Some(1) => {
            addr.do_send(ClientMessage {
                id: 0,
                room: "Main".to_owned(),
                msg: serde_json::to_string(&GameReady::new()).unwrap()
            })
        }
        Some(2) => {
            info!("{}", v["name"]);
        }
        _ => {
            info!("无可执行");
        }
    }
}*/