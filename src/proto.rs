use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SendParcel {
    GameReady(u8),
    SendKeyboard(Vec<Vec<u8>>),
    GetGameStatus(usize),
    SendGameStatus(String),
    GameStart(usize),
}
