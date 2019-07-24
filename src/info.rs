extern crate reqwest;

use serde::{Deserialize};
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    query: String,
    country: String,
    country_code: String,
    city: String,
    lat: f32,
    lon: f32,
    isp: String,
    org: String,
    r#as: String,
    reverse: String,
    mobile: bool,
    proxy: bool,
}


impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let query = format!("Address : {}",self.query);
        let reverse = format!("Reverse DNS : {}",self.reverse);
        let address = format!("Host in : {} - {}",self.country, self.city);
        let coord = format!("Coord : {} {}", self.lon, self.lat);
        let org = format!("Organisation : {}", self.org);
        let isp = format!("ISP : {}", self.isp);
        let fai = format!("AS : {}", self.r#as);

        write!(f, "\n{}\n{}\n\n{}\n{}\n\n{}\n{}\n{}\n", query, reverse, address, coord, org, isp, fai)
    }
}

pub fn search(address: &str) -> Result<Info, reqwest::Error> {
    let data: Info = reqwest::get(format!("http://ip-api.com/json/{}?fields=245759", address).as_str())?
        .error_for_status()?
        .json()?;
    Ok(data)
}