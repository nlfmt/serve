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

    #[arg(short, long, default_value_t = false)]
    pub qr: bool,

    #[arg(short = 'u', long, default_value_t = false)]
    pub upload: bool,

    #[arg(short = 's', long, default_value_t = false)]
    pub symlinks: bool,
    
    #[arg(long, default_value_t = false)]
    pub allow_rename: bool,
    #[arg(long, default_value_t = false)]
    pub allow_delete: bool,


    #[arg(
        short,
        long,
        value_name = "USER:PASS",
        help = "specify a username and password",
        value_parser(Auth::from)
    )]
    pub auth: Vec<Auth>,

    #[arg(
        long,
        help = "file with logins separated by newlines",
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
