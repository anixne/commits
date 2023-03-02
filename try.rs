use std::process::Command;

fn main() {
    let name = "Alice";
    let output = Command::new(format!("echo Hello, {}!", name))
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}