use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;
use std::env;


fn main(){
    loop {

        let cwd = env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("?"));
        print!("{} >>> ", cwd.display());

        if let Err(e) = io::stdout().flush() {
            eprintln!("vst: failed to flush stdout: {e}");
        }


        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // Prevents potentially looping forever
                println!();
                break;
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("vst: failed to read input: {e}");
                continue;
            }
        }
    
        // Trim to cleanup junk read_line left
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" | "quit" => break,

            "cd" => {
                let target = args.first().copied().unwrap_or("~");
                let path = if target == "~" {
                    env::var("HOME")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| PathBuf::from("/"))
            } else {
                PathBuf::from(target)
                };
                if let Err(e) = env::set_current_dir(&path) {
                    eprintln!("cd: {}: {e}", path.display());
                }
                continue;              
        }


            "help" => {
                println!("vst - the tiny rust shell");
                continue;
            }

            _ => {}
        }


        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();


        // Prevents shell from accepting another commad until original command completes
        child.wait();
    }
}
