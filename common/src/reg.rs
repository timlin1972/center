use serde::{Deserialize, Serialize};

pub const PATH: &str = "/reg";
pub const PRINT: &str = "/reg/print";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reg {
    pub ip: String,
    pub port: u16,
    pub path: String,
}
