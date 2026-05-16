use crate::error::RshError;
use crate::parser::Pipeline;
use std::io;
use std::process::{Command, Stdio};

pub fn run(name: &str, args: &[&str]) -> Result<i32, RshError> {
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
    
    Ok(status.code().unwrap_or(1))
}

pub fn run_pipeline(pipeline: &Pipeline) -> Result<i32, RshError> {
    let cmds = &pipeline.commands;
    let count = cmds.len();

    let mut children = Vec::new();

    let mut previous_stdout: Option<std::process::ChildStdout> = None;

    for (i, cmd) in cmds.iter().enumerate() {
        let is_last = i == count -1;

        let stdin = match previous_stdout.take() {
            Some(stdout)    => Stdio::from(stdout),
            None            => Stdio::inherit(),
        };

        let stdout = if is_last {
            Stdio::inherit()
        } else {
            Stdio::piped()
        };

        let mut child = Command::new(&cmd.name)
            .args(&cmd.args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()
            .map_err(|e| {
                if e.kind() == io::ErrorKind::NotFound {
                    RshError::CommandNotFound(cmd.name.clone())
                 } else {
                    RshError::SpawnFailed(cmd.name.clone(), e)
                }
            })?;

        if !is_last {
            previous_stdout = child.stdout.take();
        }

        children.push((cmd.name.clone(), child));
    }

    let mut last_code = 0i32;
    for (name, mut child) in children {
        let status = child.wait()
            .map_err(|e| RshError::WaitFailed(name.clone(), e))?;
        last_code = status.code().unwrap_or(1);
    }


    Ok(last_code)
}






















