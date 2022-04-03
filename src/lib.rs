mod config;
mod utils;

// -- IP Address calls --
/// Return either the callers IPv4 or IPv6 address
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
    use std::net::{IpAddr};

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
}
