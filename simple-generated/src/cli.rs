use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

#[derive(Debug, Parser)]
pub struct Cli {
    /// Verbosity flags (--verbose / --quiet) for logging
    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,

    /// The IP address to bind to
    #[arg(long, short, default_value_t = Ipv4Addr::LOCALHOST.into())]
    pub ip: IpAddr,

    /// The port to bind to
    #[arg(long, short, default_value = "3000")]
    pub port: u16,

    /// Open the browser after starting the server
    #[arg(long)]
    pub open: bool,

    /// The directory to load data from
    #[arg(long, default_value = ".data")]
    pub data_dir: PathBuf,
}

pub fn args() -> Cli {
    Cli::parse()
}
