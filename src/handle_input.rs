use std::{net::{Ipv4Addr}};
use std::error::Error;

pub fn make_into_ip(args: Vec<String>) -> Result<Ipv4Addr, Box<dyn Error>> {
    // checking if we have enough arguments to form an IP
    if args.len() == 1 {
        return Err("Not enough arguments to run this program".into())
    } 

    // dividing IP into substring of numbers
    let divided: Vec<&str> = args[1].split(".").collect(); 
    
    if divided.len() > 4 {
        return Err("Invalid IP Address".into())
    }
    
    let mut octets: Vec<u8> = vec![];

    for value in divided.iter() {
        octets.push(value.parse()?);
    }

    let ip = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);
    
    return Ok(ip)
}
