use std::net::{IpAddr, Ipv4Addr};

use clap::{command, Parser};

const TETHER_HOST: std::net::IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Default JSON file path to load on startup; no panic if not found
    #[arg(long = "load", default_value_t = String::from("./widgets.json"))]
    pub json_load: String,

    /// Flag to disable Tether connection
    #[arg(long = "tether.disable")]
    pub tether_disable: bool,

    /// The IP address of the Tether MQTT broker (server)
    #[arg(long = "tether.host", default_value_t=TETHER_HOST)]
    pub tether_host: std::net::IpAddr,

    #[arg(long = "loglevel",default_value_t=String::from("info"))]
    pub log_level: String,
}
