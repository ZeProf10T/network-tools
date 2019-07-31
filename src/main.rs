extern crate console;
extern crate dialoguer;

mod ipv4;
mod info;
mod scanner;

use dialoguer::{theme::ColorfulTheme, Select, Input};
use console::Style;

fn main() {
    let selections = &[
        "Address information",
        "Port scanner",
        "IP calculator",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a tool")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selection {
        0 => {
            let address: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Address")
                .interact()
                .unwrap();

            match info::search(address.as_str()) {
                Ok(data) => println!("{}",data),
                Err(err) => eprintln!("{}",err),
            }

        },
        1 => {
            let address: String= Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Address")
                .interact()
                .unwrap();

            scanner::scan(address.as_str())
        },
        2 => {
            let address: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Address")
                .interact()
                .unwrap();

            let mask: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Mask")
                .interact()
                .unwrap();

            ipv4::calculator(address.as_str(), mask.as_str());
        },
        _ => {},
    }

}


