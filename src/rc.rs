use std::fs;
use std::path::PathBuf;
use std::env;
use crate::{builtins, execute, parser};
use crate::error::RshError;

fn rc_path(filename: &str) -> Result<PathBuf, RshError> {
    let home = env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| RshError::RcNotFound(filename.to_string()))?;
    Ok(home.join(filename))
}

pub fn load(filename: &str) {
    let path = match rc_path(filename) {
        Ok(p) => p,
        Err(e) => { eprintln!("{e}"); return; }
    };

    // continues if path doesnt exist -- no .rshrc or .rsh_profile is normal
    if !path.exists() {
        return; 
    }

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", RshError::RcReadFailed(path.display().to_string(), e));
            return;
        }
    };

    for (i, line) in contents.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let pipeline = match parser::parse(line) {
            Some(c) => c,
            None => continue,
        }; 

    let cmd = &pipeline.commands[0];
    let args: Vec<&str> = cmds.args.iter().map(|s| s.as_str()).collect();
    match builtins::run(cmd.name.as_str(), &args) {
        Some(Ok(())) => {}
        Some(Err(e)) => eprintln!("rsh: {filename}:{}: {e}", i + 1),
        None => {
            if let Err(e) = execute::run(cmd.name.as_str(), &args) {
                eprintln!("rsh: {filename}:{}: {e}", i + 1);
            }
        }
    }
  }
}













