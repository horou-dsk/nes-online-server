use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ControllerIns {
    mid: u8,
    pub keys: Vec<u8>
}
