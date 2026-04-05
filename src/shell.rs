// src/shell.rs

use crate::{builtins, execute, input, parser, rc};
use std::env;
use std::path::PathBuf;

pub fn run() {

    rc::load(".rsh_profile");
    rc::load(".rshrc");


    loop {
        let prompt = build_prompt();

        let line = match input::read_line(&prompt) {
            Ok(Some(line)) => line,
            Ok(None) => { println!(); break; } // Ctrl+D
            Err(e) => { eprintln!("{e}"); continue; }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let cmd = match parser::parse(trimmed) {
            Some(c) => c,
            None => continue,
        };

        let expanded = builtins::expand_alias(cmd.name)
            .map(|val| format!("{val} {}", cmd.args.join(" ")))
            .unwrap_or_else(|| format!("{} {}", cmd.name, cmd.args.join(" ")));
            
        let expanded = expanded.trim().to_string();
        
        let cmd = match parser::parse(&expanded) {
            Some(c) => c,
            None => continue,
        };
        
        match builtins::run(cmd.name, &cmd.args) {
            Some(Ok(())) => {}
            Some(Err(e)) => eprintln!("{e}"),
            None => {
                if let Err(e) = execute::run(cmd.name, &cmd.args) {
                    eprintln!("{e}")
                }
            }
        }
    }
}

fn build_prompt() -> String {
    let cwd = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("?"));
    format!("{} ~# ", cwd.display())
}
