use reqwest;
use serde::Deserialize;

use super::config;

/// Struct for parsing results from the seeip geographical API
#[derive(Deserialize)]
#[serde(default)]
pub struct GeoInfo {
    pub ip: String,
    pub country: String,
    pub city: String,
    pub region: String,
    pub organization: String,
    pub timezone: String,
    pub country_code3: String,
    pub postal_code: String,
    pub continent_code: String,
    pub region_code: String,
    pub country_code: String,
    pub dma_code: i32,
    pub area_code: i32,
    pub offset: i32,
    pub asn i32,
    pub longitude: f64,
    pub latitude: f64,
}

impl Default for GeoInfo {
    fn default() -> GeoInfo {
        GeoInfo {
            ip: String::from(""),
            country: String::from(""),
            city: String::from(""),
            region: String::from(""),
            organization: String::from(""),
            timezone: String::from(""),
            country_code3: String::from(""),
            postal_code: String::from(""),
            continent_code: String::from(""),
            region_code: String::from(""),
            country_code: String::from(""),
            dma_code: 0,
            area_code: 0,
            offset: 0,
            asn: 0,
            longitude: 0.0,
            latitude: 0.0,
        }
    }
}

fn is_resp_successful(
    resp: &reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    if !resp.status().is_success() {
        return Err(format!(
            "Non-successful status code received: {}",
            resp.status().as_str()
        ))?;
    }
    Ok(())
}

/// Call the IP address endpoint and return the IP as a String
pub fn call_addr(seeip_cfg: config::Config) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(seeip_cfg.url)?;
    is_resp_successful(&resp)?;
    Ok(resp.text()?)
}

/// Call the Geographical endpoint and return info in struct
pub fn call_geo(seeip_cfg: config::Config) -> Result<GeoInfo, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(seeip_cfg.url)?;
    is_resp_successful(&resp)?;
    let geo_resp: GeoInfo = resp.json()?;
    Ok(geo_resp)
}
