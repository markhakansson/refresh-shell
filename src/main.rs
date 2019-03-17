use std::io;
use std::io::{Write};
use std::process;
use std::process::Command;
use std::path::Path;
use std::env;
use std::str::SplitWhitespace;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();
        
        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
       
        // Split input
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        
        read_command(command, args);

    }
}

// Checks the input and runs the given command
fn read_command(command: &str, args: SplitWhitespace) {
    match command {
        "cd" => {
            let direction = args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(direction);
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }
        },

        "exit" => process::exit(1),

        command => {
            // Run command
            let mut child = Command::new(command)
                .args(args)
                .spawn();

            match child {
                Ok(mut child) => { child.wait(); },
                Err(e) => eprintln!("{}", e),
            }
        }
    }
}
