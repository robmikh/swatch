use std::{process::Command, time::Instant};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Invalid number of arguments");
        return;
    }
    let mut command = &mut Command::new(&args[1]);
    if args.len() > 2 {
        command = command.args(&args[2..]);
    }
    let start = Instant::now();
    let mut process = command.spawn().expect("Failed to execute process");
    let _status = process.wait().expect("Failed to wait on the child process");
    let length = start.elapsed();
    let total_seconds = length.as_secs();
    let total_minutes = total_seconds / 60;
    let hours_component = total_minutes / 60;
    let minutes_component = total_minutes - (hours_component * 60);
    let seconds_component = total_seconds - (total_minutes * 60);
    let millis_component = length.as_millis() - (total_seconds as u128 * 1000);
    println!("");
    let mut message = String::new();
    if hours_component > 0 {
        message.push_str(&format!("{} hours ", hours_component));
    }
    if minutes_component > 0 {
        message.push_str(&format!("{} minutes ", minutes_component));
    }
    if seconds_component > 0 {
        message.push_str(&format!("{} seconds ", seconds_component));
    }
    message.push_str(&format!("{} milliseconds", millis_component));
    println!("{} elapsed", message);
}
