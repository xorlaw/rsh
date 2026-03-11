
use crate::error::RshError;
use std::env;
use std::path::PathBuf;

// Runs a builtin if the command matches one.
// Returns None if it's not a builtin.
pub fn run(name: &str, args: &[&str]) -> Option<Result<(), RshError>> {
    match name {
        "exit" | "quit" => std::process::exit(0),

        "cd" => Some(cd(args)),

        "export" => Some(export(args)),

        "alias" => {
            // placeholder for now
            Some(Ok(()))
        }



        "help" => {
            println!("rsh - the small, secure shell");
            println!("builtins: cd, exit, quit, help");
            Some(Ok(()))
        }

        _ => None,
    }
}

fn cd(args: &[&str]) -> Result<(), RshError> {
    let target = args.first().copied().unwrap_or("~");

    let path = if target == "~" {
        env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/"))
    } else {
        PathBuf::from(target)
    };

    env::set_current_dir(&path)
        .map_err(|e| RshError::CdFailed(path.display().to_string(), e))
}

fn export(args: &[&str]) -> Result<(), RshError> {
    for arg in args {
        if let Some((key, val)) = arg.split_one('=') {
            if key.is_empty() {
                eprintln!("rsh: export: invalid variable name");
                continue;
            }
            env::set_var(key, val);
        } else {
            eprintln!("rsh: export: expected KEY=VALUE, instead got '{arg}'")
        }
    }
    Ok(())
}


