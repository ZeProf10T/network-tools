extern crate clap;
use clap::{Arg, App};

mod ipv4;
mod info;
mod scanner;


fn main() {
    let matches = App::new("Network tools")
        .version("0.1.0")
        .author("Léo Huteau <huteau890@gmail.com>")
        .about("IP calculator, IP information, ...")
        .arg(Arg::with_name("operation")
            .required(true)
            .value_name("operation")
            .help("info | calculator ")
        )
        .arg(Arg::with_name("address")
            .required(true)
            .value_name("address")
            .help("192.168.0.25 | github.com")
        )
        .arg(Arg::with_name("mask")
            .required(false)
            .value_name("mask")
            .help("255.255.255.0 | 24")
        )
        .get_matches();

    match matches.value_of("operation").unwrap() {
        "calculator" | "calc" => {
                if matches.is_present("address") && matches.is_present("mask") {
                    ipv4::calculator(
                        matches.value_of("address").unwrap(),
                        matches.value_of("mask").unwrap()
                    );
                } else {
                    eprintln!("Mask required in subnet calculator mode")
                }
        },
        "information" | "info" => {
            match info::search(matches.value_of("address").unwrap()) {
                Ok(data) => println!("{}",data),
                Err(err) => eprintln!("{}",err),
            }
        },
        "scanner" | "scan" => {
            scanner::scan(matches.value_of("address").unwrap())
         }
        _ => eprintln!("Choose type of operation, ex: calculator"),
    }





}


