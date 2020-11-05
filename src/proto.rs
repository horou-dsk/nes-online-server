use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SendParcel {
    GameReady(u8),
    SendKeyboard(Vec<u8>),
    GetGameStatus(u8),
}
