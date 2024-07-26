use std::process::Command;
use std::io::{stdin, stdout};
use std::io::Write;
use std::path::Path;
use std::env;

fn main() {
    println!("Welcome to the Rust shell");
    loop{
        print!("> ");

        // Flush the output to the console
        let _ = stdout().flush();

        let mut input = String::new();

        // Read the input from the user until the user presses enter, unwrap handles erroe
        stdin().read_line(&mut input).unwrap();

        // Trim the input to remove the leading and trailing whitespaces
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path :: new(new_dir);
                if let Err(e) = env::set_current_dir(&root){
                    eprintln!("{}", e);
                }
            },
            "exit" => {
                return
            },
            command => {
                let child = Command :: new(command)
                    .args(args)
                    .spawn();
                
                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
                
            }
        };

        // Execute the command
        // let mut child = Command :: new(command)
        //     .args(args)
        //     .spawn()
        //     .unwrap();
        
        // // Wait for the command to finish
        // let _ = child.wait();
    }
}
