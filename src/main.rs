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
        let args = parts;

        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .unwrap();


        // Prevents shell from accepting another commad until original command completes
        child.wait();
    }
}
