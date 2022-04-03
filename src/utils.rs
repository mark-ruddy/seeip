use serde::{Deserialize};
use reqwest;

use super::config;

/// Struct for parsing results from the seeip geographical API
/// See https://seeip.org/ for details
#[derive(Deserialize)]
pub struct GeoInfo {
    pub ip: String,
    pub organization: String,
    pub city: String,
    pub region: String,
    pub dma_code: String,
    pub area_code: String,
    pub timezone: String,
    pub offset: String,
    pub longitude: f64,
    pub country_code3: String,
    pub postal_code: String,
    pub continent_code: String,
    pub country: String,
    pub region_code: String,
    pub country_code: String,
    pub latitude: f64,
}

/// Call the IP address endpoint and return the IP as a String
pub fn call_addr(seeip_cfg: config::Config) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::blocking::get(seeip_cfg.url)?.text()?)
}

/// Call the Geographical endpoint and return info in struct
pub fn call_geo(seeip_cfg: config::Config) -> Result<GeoInfo, Box<dyn std::error::Error>> {
    let geo_resp: GeoInfo = reqwest::blocking::get(seeip_cfg.url)?.json()?;
    Ok(geo_resp)
}
