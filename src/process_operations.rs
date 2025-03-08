use std::env;
use std::process::{exit, Command};

pub fn restart_process() {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let mut command = Command::new(current_exe);
    command.current_dir(current_dir);

    match command.spawn() {
        Ok(_) => {
            // Successfully spawned new process, now exit the current one
            exit(0);
        },
        Err(e) => {
            // Handle error - couldn't spawn new process
            eprintln!("Failed to restart process: {}", e);
            exit(1);
        }
    }
}