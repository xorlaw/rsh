use crate::{builtins, execute, input, parser, rc};
use std::env;
use std::path::PathBuf;
use std::collections::HashSet;
use std::time::Instant;


pub fn run() {
    env::set_var("STARSHIP_SHELL", "bash");
    let mut exit_code: i32 = 0;
    let mut duration_ms: u64 = 0;


    rc::load(".rsh_profile");
    rc::load(".rshrc");
    loop {
        let prompt = build_prompt(exit_code, duration_ms);
        let line = match input::read_line(&prompt) {
            Ok(Some(line))  => line,
            Ok(None)        => { println!(); break; }
            Err(e)          => { eprintln!("{e}"); continue; }
        };


        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let pipeline = match parser::parse(trimmed) {
            Some(p) => p,
            None    => continue,
        };

        if pipeline.commands.len() > 1 {
            let start = Instant::now();
            exit_code = match execute::run_pipeline(&pipeline) {
                Ok(code)    => code,
                Err(e)      => { eprintln!("{e}"); 1 }
            };
            duration_ms = start.elapsed().as_millis() as u64;
    
            continue;
        }

        let cmd = &pipeline.commands[0];
        let expanded = builtins::expand_alias(&cmd.name, &mut HashSet::new())
            .map(|val| format!("{val} {}", cmd.args.join(" ")))
            .unwrap_or_else(|| format!("{} {}", cmd.name, cmd.args.join(" ")));

        let pipeline = match parser::parse(expanded.trim()) {
            Some(p) => p,
            None    => continue,
        };

        if pipeline.commands.len() > 1 {
            let start = Instant::now();
            exit_code = match execute::run_pipeline(&pipeline) {
                Ok(code)    => code,
                Err(e)      => { eprintln!("{e}"); 1 }
            };
            duration_ms = start.elapsed().as_millis() as u64;
            continue;
        }

        let cmd = &pipeline.commands[0];
        let args: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();

        let start = Instant::now();
        exit_code = match builtins::run(&cmd.name, &args) {
            Some(Ok(()))    => 0,
            Some(Err(e))    => { eprintln!("{e}"); 1 }
            None => {
                match execute::run(&cmd.name, &args) {
                    Ok(code)    => code,
                    Err(e)      => { eprintln!("{e}"); 1 }
                }
            }
        };
        duration_ms = start.elapsed().as_millis() as u64;
    }
}

fn build_prompt(exit_code: i32, duration_ms: u64) -> String {
    std::process::Command::new("starship")
        .arg("prompt")
        .arg(format!("--status={exit_code}"))
        .arg(format!("--cmd-duration={duration_ms}"))
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| {
            let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("?"));
            format!("{} ~# ", cwd.display())
        })


}





