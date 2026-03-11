use crate::error::RshError;
use std::env;
use std::path::PathBuf;
// use std::collections::HashMap;
// use std::sync::Mutex; needed for aliases which i'll probably add later




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
    if args.is_empty() {
        eprintln!("rsh: export: expected KEY=VALUE");
        return Ok(());
    }

    for arg in args {
        match arg.split_once('=') {
            Some((key, val)) if !key.is_empty() => {
                env::set_var(key, val);
            }
            Some((_, _)) => {
                eprintln!("rsh: export: invalid variable name in '{arg}'");
            }
            None => eprintln!("rsh: export: expected KEY=VALUE, instead got '{arg}'"),
        }
    }
    Ok(())
}


