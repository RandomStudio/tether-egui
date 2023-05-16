use std::net::{IpAddr, Ipv4Addr};

use clap::{command, Parser};

const TETHER_HOST: std::net::IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Default JSON file path to load on startup; no panic if not found
    pub json_load: Option<String>,

    /// Flag to disable Tether connection
    #[arg(long = "tether.disable")]
    pub tether_disable: bool,

    /// The IP address of the Tether MQTT broker (server)
    #[arg(long = "tether.host", default_value_t=TETHER_HOST)]
    pub tether_host: std::net::IpAddr,

    /// Optional username for MQTT Broker
    #[arg(long = "tether.username")]
    pub tether_username: Option<String>,

    /// Optional password for MQTT Broker
    #[arg(long = "tether.password")]
    pub tether_password: Option<String>,

    /// Flag to enable "continuous mode" on startup; sometimes improves message log
    /// latency, at the cost of higher-than-normal CPU usage
    #[arg(long = "continuous")]
    pub continuous_mode: bool,

    #[arg(long = "loglevel",default_value_t=String::from("info"))]
    pub log_level: String,
}
