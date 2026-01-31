use std::{process, env};
// use std::net;

mod handle_input;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let ip = match handle_input::make_into_ip(args) {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error parsing IP: {e}");
            process::exit(1);
        }
    };

    print!("The IP Address you provided is: {:?}", ip);
}


