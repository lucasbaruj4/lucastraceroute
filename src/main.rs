use std::{process, env};
use std::net::{self, IpAddr, Ipv4Addr, SocketAddr, TcpStream};

mod handle_input;

fn main() {
    // getting the IP from the user's argument
    let args: Vec<String> = env::args().collect(); 

    // make this port_number definition into a function in the handle_input module - TODO!
    let port_number = args[2].clone();
    let ip = match handle_input::make_into_ip(args) {
        Ok(ip) => ip,
        Err(e) => {
            println!("Error parsing IP: {e}");
            process::exit(1);
        }
    };


    let final_address = ip.to_string() + ":" + &port_number;

    // establishing TCP connection
    let mut stream = TcpStream::connect(final_address);
    println!("Established connection to: {ip}:{port_number}");


    print!("The IP Address you provided is: {:?}", ip);
}



