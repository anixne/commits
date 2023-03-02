use std::fs::File;

fn main() {
    let name = "hey";
    File::create(format!("new_{}", name));
    println!("Done");
}