mod ipv4;
use ipv4::Address;

fn main() {

    match Address::is_valid_ipv4("192.168.255.5") {
        Ok(ip) => {
            match Address::is_valid_mask("255.255.255.0") {
                Ok(mask) => { println!("{}", Address { ip, mask }) },
                Err(err) => { eprintln!("{}", err) }
            }
        },
        Err(err) => { eprintln!("{}", err) }
    };

}
