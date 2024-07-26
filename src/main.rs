use std::process::Command;
use std::io::{stdin, stdout};
use std::io::Write;
use std::path::Path;
use std::env;
use std::process::{Stdio};
use std::process::Child;

fn main() {
    println!("Welcome to the Rust shell");
    loop{
        print!("> ");

        // Flush the output to the console
        let _ = stdout().flush();

        let mut input = String::new();

        // Read the input from the user until the user presses enter, unwrap handles erroe
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {

            // Trim the input to remove the leading and trailing whitespaces
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path :: new(new_dir);
                    if let Err(e) = env::set_current_dir(&root){
                        eprintln!("{}", e);
                    }
                    prev_command = None;
                },
                "exit" => {
                    return
                },
                command => {
                    let stdin = prev_command.map_or(
                        Stdio :: inherit(),
                        |output: Child| Stdio :: from(output.stdout.unwrap())
                    );
                    let stdout = if commands.peek().is_some(){
                        // Create a pipe if the current command has a next command
                        Stdio :: piped()
                    }
                    else {
                        Stdio :: inherit()
                    };

                    let child = Command :: new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    
                    match child {
                        Ok(child) => {
                            prev_command = Some(child);
                        },
                        Err(e) => {
                            prev_command = None;
                            eprintln!("{}", e);
                        },
                    };
                    
                }
            }
            
        }
        if let Some(mut final_command) = prev_command {
            // Wait for the child process to finish
            let _ = final_command.wait();
        }
    }
}
