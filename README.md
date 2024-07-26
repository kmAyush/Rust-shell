# Rust Shell

A shell program easy to run, also supports special command ```cd```, ```end``` and  ```|``` pipeline
<br>

How to run
----

Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Install Cargo
```
curl https://sh.rustup.rs -sSf | sh
```
Run cargo
```
cd Rust-shell
cargo run
```
<br>

Example
---
```console
(base) ayush@admin:~/Downloads/ayushkm/Rust/shell$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/shell`
Welcome to the Rust shell
> ls
Cargo.lock  Cargo.toml	src  target
> cat src/main.rs | grep child
                    let child = Command :: new(command)
                    match child {
                        Ok(child) => {
                            prev_command = Some(child);
            // Wait for the child process to finish
> exit
(base) ayush@admin:~/Downloads/ayushkm/Rust/shell$ 
```

