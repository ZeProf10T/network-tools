use rayon::prelude::*;
use std::net::TcpStream;
use std::time::{Duration};


pub fn scan(address: &str) {
    for i in 0..32{
        let ports: Vec<i32> = (i*32..i*32+32).collect();
        let opens: Vec<_> = ports.par_iter()
            .filter(|i|
                match TcpStream::connect_timeout(&format!("{}:{}",address,i).parse().unwrap(),Duration::new(0,200000000)) {
                    Ok(_) => return true,
                    Err(_) => return false,
                }

            )
            .collect();

        for open in opens {
            println!("Port {} is open", open);
        }
    }
}