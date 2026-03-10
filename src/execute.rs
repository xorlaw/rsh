// src/execute.rs

use crate::error::RshError;
use std::io;
use std::process::Command;

pub fn run(name: &str, args: &[&str]) -> Result<(), RshError> {
    let mut child = Command::new(name)
        .args(args)
        .spawn()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                RshError::CommandNotFound(name.to_string())
            } else {
                RshError::SpawnFailed(name.to_string(), e)
            }
        })?;

    let status = child
        .wait()
        .map_err(|e| RshError::WaitFailed(name.to_string(), e))?;

    if !status.success() {
        if let Some(code) = status.code() {
            eprintln!("rsh: '{name}' exited with code {code}");
        }
    }

    Ok(())
}
