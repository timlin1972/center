use actix_web::{HttpResponse, Responder};
use log::{error, info};
use std::io::{Error, ErrorKind};
use std::process::Command;

fn run_command(command: &str, args: &[&str]) -> std::io::Result<()> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    match cmd.output() {
        Ok(output) => {
            if output.status.success() && output.stderr.is_empty() {
                return Ok(());
            }
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
        Err(error) => Err(error),
    }
}

pub async fn get() -> impl Responder {
    info!(">>> recv: get");

    match run_command("shutdown", &["-h", "now"]) {
        Ok(_) => HttpResponse::Ok().body("Ok"),
        Err(e) => {
            error!(">>> Failed to run shutdown: {e:?}");

            match run_command("sudo", &["shutdown", "-h", "now"]) {
                Ok(_) => HttpResponse::Ok().body("Ok"),
                Err(e) => {
                    error!(">>> Failed to run sudo shutdown: {e:?}");
                    HttpResponse::Ok().body("Failed")
                }
            }
        }
    }
}
