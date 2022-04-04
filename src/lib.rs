//! Rust Wrapper for the IP Info <https://seeip.org/> API
//! # Examples
//! All of the below examples support both IPv4 and IPv6 addresses.  
//! If you specifically want to use IPv4 or IPv6 use their `_v4()` and `_v6()` equivalents.
//! ### Get the caller's IP address
//! ```
//! let my_ip = seeip::get_ip().unwrap();
//! println!("My IP: {}", my_ip);
//! ```
//! ### Get the caller's geographical information
//! ```
//! let my_geo_info = seeip::get_caller_geo().unwrap();
//! // Fields in GeoInfo can default out to empty values if not available from the API
//! println!("Country my IP matches: {}", my_geo_info.country);
//! ```
//! ### Get geographical information for an IP address
//! ```
//! let geo_info = seeip::get_geo("208.67.222.222").unwrap();
//! println!("Country matching this IP: {}", geo_info.country);
//! ```

pub mod config;
pub mod utils;

// -- IP Address calls --
/// Return the callers IPv4 or IPv6 address
pub fn get_ip() -> Result<String, Box<dyn std::error::Error>> {
    let seeip_cfg = config::default_config();
    Ok(utils::call_addr(seeip_cfg)?)
}

/// Return the callers IPv4 address
pub fn get_ip_v4() -> Result<String, Box<dyn std::error::Error>> {
    let seeip_cfg = config::ipv4_config();
    Ok(utils::call_addr(seeip_cfg)?)
}

/// Return the callers IPv6 address
pub fn get_ip_v6() -> Result<String, Box<dyn std::error::Error>> {
    let seeip_cfg = config::ipv6_config();
    Ok(utils::call_addr(seeip_cfg)?)
}

// -- GeoIP calls --
/// Return geographical info from the callers IPv4 or IPv6 address
pub fn get_caller_geo() -> Result<utils::GeoInfo, Box<dyn std::error::Error>> {
    let seeip_cfg = config::geo_default_config();
    Ok(utils::call_geo(seeip_cfg)?)
}

/// Return geographical info from the callers IPv4 address
pub fn get_caller_geo_v4() -> Result<utils::GeoInfo, Box<dyn std::error::Error>> {
    let seeip_cfg = config::geo_ipv4_config();
    Ok(utils::call_geo(seeip_cfg)?)
}

/// Return geographical info from the callers IPv6 address
pub fn get_caller_geo_v6() -> Result<utils::GeoInfo, Box<dyn std::error::Error>> {
    let seeip_cfg = config::geo_ipv6_config();
    Ok(utils::call_geo(seeip_cfg)?)
}

/// Return geographical info for a passed IPv4 or IPv6 address
pub fn get_geo(ip_addr: &str) -> Result<utils::GeoInfo, Box<dyn std::error::Error>> {
    let seeip_cfg = config::geo_default_config();
    let ip_seeip_cfg = config::cfg_add_ip(seeip_cfg, ip_addr);
    Ok(utils::call_geo(ip_seeip_cfg)?)
}

/// Return geographical info for a passed IPv4 address
pub fn get_geo_v4(ip_addr: &str) -> Result<utils::GeoInfo, Box<dyn std::error::Error>> {
    let seeip_cfg = config::geo_ipv4_config();
    let ip_seeip_cfg = config::cfg_add_ip(seeip_cfg, ip_addr);
    Ok(utils::call_geo(ip_seeip_cfg)?)
}

/// Return geographical info for a passed IPv6 address
pub fn get_geo_v6(ip_addr: &str) -> Result<utils::GeoInfo, Box<dyn std::error::Error>> {
    let seeip_cfg = config::geo_ipv6_config();
    let ip_seeip_cfg = config::cfg_add_ip(seeip_cfg, ip_addr);
    Ok(utils::call_geo(ip_seeip_cfg)?)
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    // -- IP Address calls tests --
    #[test]
    fn get_ip_valid() {
        match super::get_ip() {
            Ok(ip) => match ip.parse::<IpAddr>() {
                Ok(_) => assert!(true),
                Err(_) => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_ip_v4_valid() {
        match super::get_ip_v4() {
            Ok(ip) => match ip.parse::<IpAddr>() {
                Ok(ip) => assert_eq!(ip.is_ipv4(), true),
                Err(_) => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_ip_v6_valid() {
        match super::get_ip_v6() {
            Ok(ip) => match ip.parse::<IpAddr>() {
                Ok(ip) => assert_eq!(ip.is_ipv6(), true),
                Err(_) => assert!(false),
            },
            Err(_) => assert!(false),
        }
    }

    // -- GeoIP calls tests --
    #[test]
    fn get_caller_geo_valid() {
        match super::get_caller_geo() {
            // caller tests don't check any info as each caller can be completely different
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_caller_geo_v4_valid() {
        match super::get_caller_geo_v4() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    // Commented out flaky test since many hosts(e.g. Github Actions) don't have a caller IPv6 address
    /*#[test]
    fn get_caller_geo_v6_valid() {
        match super::get_caller_geo_v6() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }*/

    #[test]
    fn get_geo_valid() {
        match super::get_geo("208.67.222.222") {
            Ok(info) => assert_eq!(info.country_code, "US"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_geo_v4_valid() {
        match super::get_geo_v4("208.67.222.222") {
            Ok(info) => assert_eq!(info.country_code, "US"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_geo_v6_valid() {
        match super::get_geo_v6("2620:0:ccc::2") {
            Ok(info) => assert_eq!(info.country_code, "US"),
            Err(_) => assert!(false),
        }
    }
}
