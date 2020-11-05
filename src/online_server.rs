use std::collections::{HashMap, HashSet};
use rand::rngs::ThreadRng;
use rand::Rng;
use actix::prelude::*;
use crate::proto::{SendParcel, GameReady};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

pub struct OnLineServer {
    rooms: HashMap<String, HashSet<usize>>, // 房间列表
    sessions: HashMap<usize, Recipient<Message>>, // 房间长连接
    rng: ThreadRng,
}

impl Default for OnLineServer {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());
        Self {
            rooms,
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl OnLineServer {
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }

    fn send_reply(&self, message: &str, rep_id: usize) {
        if let Some(addr) = self.sessions.get(&rep_id) {
            let _ = addr.do_send(Message(message.to_owned()));
        }
    }
}

impl Actor for OnLineServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for OnLineServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Connect Handler 连接建立");

        // notify all users in same room
        // self.send_message(&"Main".to_owned(), "Someone joined", 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // auto join session to Main room
        self.rooms
            .entry("Main".to_owned())
            .or_insert(HashSet::new())
            .insert(id);

        // send id back
        id
    }
}

impl Handler<Disconnect> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("断开");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone disconnect", 0);
        }

    }
}

/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

impl Handler<ClientMessage> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, &msg.msg, msg.id);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RoomMessage {
    pub id: usize,
    pub room: String,
}

impl Handler<RoomMessage> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: RoomMessage, _: &mut Context<Self>) {
        let room = msg.room.as_str();
        if let Some(players) = self.rooms.get(room) {
            let len = players.len() as u8;
            let message = serde_json::to_string(&SendParcel::GameReady(len)).unwrap();
            self.send_reply(message.as_str(), msg.id);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SingleMessage {
    pub id: usize,
    pub msg: String,
}

impl Handler<SingleMessage> for OnLineServer {
    type Result = ();

    fn handle(&mut self, msg: SingleMessage, _: &mut Context<Self>) {
        self.send_reply(msg.msg.as_str(), msg.id);
    }
}
