use std::fmt;
use std::ops::RangeInclusive;

pub mod convert {
    pub fn vec_to_binary(vector: &Vec<u8>) -> String {
        let mut result = String::new();
        for i in vector {
            let mut value = String::new();
            value = format!("{:b}", i);

            while value.len() < 8 {
                value = format!("{}{}", "0", &value);
            }

            result += &value;
        }

        return result;
    }

    pub fn bin_to_dec(bin: &str) -> u8 {
        return u8::from_str_radix(&bin, 2).unwrap();
    }

    pub fn binary_to_vec(address: String) -> Vec<u8>{
        let mut vector: Vec<u8> = Vec::new();
        let address: &str = address.as_str();

        for i in 0..4 {
            vector.push(bin_to_dec(&address[0+i*8..8+i*8]));
        }
        return vector;
    }

    pub fn vec_to_ip(vector: &Vec<u8>) -> String{
        let result = format!(
            "{}.{}.{}.{}",
            vector[0], vector[1], vector[2], vector[3]
        );

        return result;
    }

    pub fn bytes_and(a: String, b: String) -> String {
        let mut result = String::new();
        let mut a = a.chars();
        let mut b = b.chars();
        let _zero = "0".chars().nth(0).unwrap();
        let one = "1".chars().nth(0).unwrap();

        for _i in 0..32 {
            let temp_a = a.next().unwrap();
            let temp_b = b.next().unwrap();

            if (temp_a == one) && (temp_b == one) {
                result += "1";
            } else {
                result += "0";
            }
        }
        return result;
    }

    pub fn _bytes_or(a: String, b: String) -> String {
        let mut result = String::new();
        let mut a = a.chars();
        let mut b = b.chars();
        let zero = "0".chars().nth(0).unwrap();
        let _one = "1".chars().nth(0).unwrap();

        for _i in 0..32 {
            let temp_a = a.next().unwrap();
            let temp_b = b.next().unwrap();

            if temp_a == zero && temp_b == zero {
                result += "0";
            } else {
                result += "1";
            }
        }
        return result;
    }
}


pub struct Address {
    pub ip: Vec<u8>,
    pub mask: Vec<u8>,
}

impl Address {
    pub fn is_valid_ipv4(ip: &str) -> Result<Vec<u8>,&str> {
        let ip: Vec<&str> = ip.split(".").collect();

        if ip.len() != 4 {
            return Err("Incorrect format ! IP must be write like : X.X.X.X");
        }

        let ip: Vec<u8> = ip
            .iter()
            .map(|i| i.parse::<u8>())
            .filter_map(Result::ok)
            .collect();

        if ip.len() != 4 {
            return Err("Invalid numbers ! IP must be write like : X.X.X.X with X in the range [0;255]")
        }

        return Ok(ip);
    }

    pub fn is_valid_mask(mask: &str) -> Result<Vec<u8>,&str> {
        let range: RangeInclusive<u8> = 1 ..= 32;
        let mut mask = String::from(mask);

        match mask.parse::<u8>() {
            Ok(n) => {
                if range.contains(&n){
                    let mut t = String::new();
                    for _i in 0..n {
                        t += "1";
                    }
                    for _i in n..32 {
                        t += "0";
                    }

                    mask = convert::vec_to_ip(&convert::binary_to_vec(t));

                } else {
                    return Err("CIDR format require a mask in [1;31] range");
                }
            },
           _ => {}
        }

        let mask: Vec<&str> = mask.split(".").collect();

        if mask.len() != 4 {
            return Err("Incorrect format ! Mask must be write like : X.X.X.X");
        }

        // Delete automatically numbers > 255 and numbers < 0, because we use u8 type
        let mask: Vec<u8> = mask
            .iter()
            .map(|i| i.parse::<u8>())
            .filter_map(Result::ok)
            .collect();


        if mask.len() != 4 {
            return Err("Invalid numbers ! Mask must be write like : X.X.X.X with X in the range [0;255]")
        }

        // println!("{:?}",mask);
        return Ok(mask);
    }

    fn cidr(&self) -> String {
        let cidr = convert::vec_to_binary(&self.mask);

        return format!("{}", cidr.matches("1").count());
    }

    fn subnet_address(&self) -> Vec<u8> {
        let mask = convert::vec_to_binary(&self.mask);
        let ip = convert::vec_to_binary(&self.ip);
        let subnet = convert::bytes_and(ip, mask);

        return convert::binary_to_vec(subnet);
    }

    fn first_address(&self) -> Vec<u8> {
        let mut ip = self.subnet_address().clone();
        ip[3] += 1;
        return ip;
    }

    fn last_address(&self) -> Vec<u8> {
        let subnet_address =  convert::vec_to_binary(&self.subnet_address());
        let mask = convert::vec_to_binary(&self.mask);
        let mut mask = mask.chars();
        let zero = "0".chars().next().unwrap();

        let mut first_zero = 31;
        for i in 0..32 {
            if let Some(a) = mask.next() {
                if a == zero {
                    first_zero = i;
                    break;
                }
            }
        }

        let result = subnet_address.chars().enumerate().map(|(i,c)| if i >= first_zero && i != 31 { String::from("1") } else { c.to_string() }).collect();
        return convert::binary_to_vec(result);
    }

    fn free_address(&self) -> i32 {
        let cidr = u32::from_str_radix(&self.cidr(), 10).unwrap();
        let num: u32 = 32 - cidr;
        return 2_i32.pow(num) - 2;
    }

    fn broadcast_address(&self) -> Vec<u8> {
        let subnet_address =  convert::vec_to_binary(&self.subnet_address());
        let mask = convert::vec_to_binary(&self.mask);
        let mut mask = mask.chars();
        let zero = "0".chars().next().unwrap();

        let mut first_zero = 31;
        for i in 0..32 {
            if let Some(a) = mask.next() {
                if a == zero {
                    first_zero = i;
                    break;
                }
            }
        }

        let result = subnet_address.chars().enumerate().map(|(i,c)| if i >= first_zero  { String::from("1") } else { c.to_string() }).collect();
        return convert::binary_to_vec(result);
    }

    pub fn public(&self) -> bool {
        if self.ip[0] == 10 {
            return false;
        } else if self.ip[0] == 172 && self.ip[1] >= 16 && self.ip[1] < 32{
            return false;
        } else if self.ip[0] == 192 && self.ip[1] == 168{
            return false;
        } else {
            return true;
        }
    }

    pub fn class(&self) -> String {
        let class = match self.ip[0] {
            0 ..= 127 => "A",
            128 ..= 191 => "B",
            192 ..= 223 => "C",
            224 ..= 239 => "D",
            _ => "E",
        };

        return String::from(class);
    }


}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ip = format!("IP address : {}",convert::vec_to_ip(&self.ip));
        let mask= format!("Mask : {} / CIDR : {}",convert::vec_to_ip(&self.mask), self.cidr());

        let class = format!("This network belongs to Class {}",self.class().to_uppercase());
        let public = format!("It's a {} network", match self.public() {
            true => "public",
            false => "private",
        });



        let subnet = format!("Network address : {}",convert::vec_to_ip(&self.subnet_address()));
        let broadcast = format!("Broadcast address : {}",convert::vec_to_ip(&self.broadcast_address()));

        let first_ip = format!("First IP : {}", convert::vec_to_ip(&self.first_address()));
        let last_ip = format!("Last IP : {}",convert::vec_to_ip(&self.last_address()));
        let free_address = format!("Number of free address : {}",self.free_address());




        write!(f,"\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n{}\n",ip,mask,class,public,subnet,broadcast,first_ip,last_ip,free_address)
    }
}

pub fn calculator(address: &str, mask: &str)  {
    match Address::is_valid_ipv4(address) {
        Ok(ip) => {
            match Address::is_valid_mask(mask) {
                Ok(mask) => { println!("{}", Address { ip, mask }) },
                Err(err) => { eprintln!("{}", err) }
            }
        },
        Err(err) => { eprintln!("{}", err) }
    };
}


