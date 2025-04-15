use std::{
    net::{AddrParseError, IpAddr},
    path::Path,
    str::FromStr,
};

use clap::Parser;

use crate::auth::Auth;

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

    #[arg(short, long, default_value_t = false, help = "Show a connections QR code in the terminal")]
    pub qr: bool,

    #[arg(short = 'u', long, default_value_t = false, help = "Allow users to upload files to the served folder")]
    pub upload: bool,

    #[arg(short = 's', long, default_value_t = false, help = "Resolve symlinks instead of ignoring them (dangerous!)")]
    pub symlinks: bool,
    
    #[arg(short = 'o', long, default_value_t = false, help = "Allow users to overwrite files during upload")]
    pub overwrite: bool,
    #[arg(short = 'r', long, default_value_t = false, help = "Allow users to rename files/folders")]
    pub rename: bool,
    #[arg(short = 'd', long, default_value_t = false, help = "Allow users to delete files/folders")]
    pub delete: bool,
    #[arg(short = 'm', long, default_value_t = false, help = "Enables overwriting, renaming and deleting")]
    pub modify: bool,


    #[arg(
        short,
        long,
        value_name = "USER:PASS",
        help = "Specify a username and password",
        value_parser(Auth::from)
    )]
    pub auth: Vec<Auth>,

    #[arg(
        long,
        help = "File with logins separated by newlines",
        value_parser(parse_auth_file)
    )]
    pub auth_file: Vec<Vec<Auth>>,

    pub root_dir: Option<String>,
}

impl ServeArgs {
    pub fn auths(&self) -> Vec<Auth> {
        self.auth
            .iter()
            .chain(self.auth_file.iter().flatten())
            .cloned()
            .collect()
    }
}

fn parse_interface(s: &str) -> Result<IpAddr, AddrParseError> {
    s.parse()
}

fn parse_auth_file(s: &str) -> Result<Vec<Auth>, anyhow::Error> {
    let path = Path::new(s);

    let contents = std::fs::read_to_string(path)?;
    let auths = contents
        .lines()
        .map(Auth::from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(auths)
}
