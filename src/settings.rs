use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Default JSON file path to load on startup; no panic if not found
    pub json_load: Option<String>,

    /// Flag to disable Tether connection
    #[arg(long = "tether.disable")]
    pub tether_disable: bool,

    #[arg(long = "monitor.topic", default_value_t=String::from("#"))]
    pub monitor_topic: String,

    /// Flag to enable "continuous mode" on startup; sometimes improves message log
    /// latency, at the cost of higher-than-normal CPU usage
    #[arg(long = "continuous")]
    pub continuous_mode: bool,

    #[arg(long = "loglevel",default_value_t=String::from("info"))]
    pub log_level: String,
}
