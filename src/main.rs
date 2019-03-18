use std::io;
use std::io::{Write};
use std::process;
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();
        
        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
       
        // Split input
        let mut parts = input.trim().split_whitespace().peekable();
        if parts.peek().is_some() {
            let command = parts.next().unwrap();
            let args = parts;
            read_command(command, args);
        } 
    }
}

// Checks the input and runs the given command
fn read_command<'a,I>(command: &str, args: I) 
    where I: Iterator<Item = &'a str>,
{ 
    match command {
        "cd" => {
            cd(args);
        },

        "exit" => process::exit(0),

        command => {
            run_command(command, args);
        }
    }
}

// Runs external command
fn run_command<'a,I>(command: &str, args: I) 
    where I: Iterator<Item = &'a str>,
{
    // Run command
    let child = Command::new(command)
        .args(args)
        .spawn();

    match child {
        Ok(mut child) => { child.wait(); },
        Err(e) => eprintln!("{}", e),
    }
}

// Shell builtin cd
fn cd<'a,I>(args: I) 
    where I: Iterator<Item=&'a str>, 
{    
    let direction = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(direction);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }
}
