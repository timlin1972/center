use std::io::{Error, ErrorKind};
use std::process::Command;

pub fn run_command(command: &str, args: &[&str]) -> std::io::Result<()> {
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
