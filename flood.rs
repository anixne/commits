use std::process;
use std::{thread, time};


fn main(){

    const NUMBER_OF_COMMITS: i32 = 10;

    let mut count: i32 = 0;

    loop {
        count += 1;

        process::Command::new("touch").arg(format!("./files-rs/{}.txt", count.clone())).spawn();
        thread::sleep(time::Duration::from_millis(500))
        process::Command::new("git").arg("add").arg(format!("./files-rs/{}.txt", count.clone())).spawn();
        process::Command::new("git").arg("commit").arg("-m").arg(format!("\".{}\"", count.clone())).spawn();

        if count==NUMBER_OF_COMMITS{
            process::Command::new("git push origin main");

            // Break it blyat
            break;
        }
    }

    println!("Done!");
}