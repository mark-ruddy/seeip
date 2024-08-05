/// Specify the type of IP to be used when calling the API
pub enum IpKind {
    V4,
    V6,
    Both,
}

/// Configuration for the functions that call the API to use
pub struct Config {
    pub ip_kind: IpKind,
    pub url: String,
}

/// The default configuration which supports both IPv4 and IPv6
pub fn default_config() -> Config {
    return Config {
        ip_kind: IpKind::Both,
        url: String::from("https://api.seeip.org"),
    };
}

/// Only use IPv4 configuration
pub fn ipv4_config() -> Config {
    return Config {
        ip_kind: IpKind::V4,
        url: String::from("https://ipv4.seeip.org"),
    };
}

/// Only use IPv6 configuration
pub fn ipv6_config() -> Config {
    return Config {
        ip_kind: IpKind::V6,
        url: String::from("https://ipv6.seeip.org"),
    };
}

/// The default configuration for geo which supports both IPv4 and IPv6
pub fn geo_default_config() -> Config {
    return Config {
        ip_kind: IpKind::Both,
        url: String::from("https://api.seeip.org/geoip"),
    };
}

/// Only use IPv4 configuration for geo
pub fn geo_ipv4_config() -> Config {
    return Config {
        ip_kind: IpKind::V4,
        url: String::from("https://ipv4.seeip.org/geoip"),
    };
}

/// Only use IPv6 configuration for geo
pub fn geo_ipv6_config() -> Config {
    return Config {
        ip_kind: IpKind::V6,
        url: String::from("https://ipv6.seeip.org/geoip"),
    };
}

// Append an IP to an existing config
pub fn cfg_add_ip(mut seeip_cfg: Config, ip_addr: &str) -> Config {
    let ip_url = format!("{}/{}", seeip_cfg.url, ip_addr);
    seeip_cfg.url = ip_url;
    seeip_cfg
}
