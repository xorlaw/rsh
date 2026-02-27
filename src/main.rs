use std::io::{self, Write};
use std::process::Command;
use std::io::stdin;
use std::io::stdout;

fn main(){
    loop {
        // Flush to ensure prompt prints before read_line
        print!(">>> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

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
