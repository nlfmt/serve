use std::{
    net::{AddrParseError, IpAddr},
    str::FromStr,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ServeArgs {
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    #[arg(
        short = 'i',
        long = "interface",
        value_parser(parse_interface),
        default_value_t = IpAddr::from_str("0.0.0.0").unwrap()
    )]
    pub interface: IpAddr,
    #[arg(short, long, default_value_t = false)]
    pub qr: bool,

    #[arg(short = 'u', long, default_value_t = false)]
    pub allow_upload: bool,
    #[arg(short = 's', long, default_value_t = false)]
    pub allow_symlinks: bool,

    pub root_dir: Option<String>,
}

fn parse_interface(s: &str) -> Result<IpAddr, AddrParseError> {
    s.parse()
}
