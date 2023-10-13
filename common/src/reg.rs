use log::info;
use serde::{Deserialize, Serialize};

pub const PATH: &str = "/reg";
pub const PRINT: &str = "/reg/print";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reg {
    pub ip: String,
    pub port: u16,
    pub path: String,
}

pub async fn request(
    ip: &str,
    port: u16,
    path: &str,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let reg = Reg {
        ip: ip.to_owned(),
        port,
        path: path.to_owned(),
    };

    let response = reqwest::Client::new()
        .post(format!(
            "http://{}:{}{}",
            super::CENTER_IP,
            super::CENTER_PORT,
            super::reg::PATH
        ))
        .json(&reg)
        .send()
        .await?;

    info!(">>> send: reg {ip}::{port}/{path}");

    Ok(response)
}
