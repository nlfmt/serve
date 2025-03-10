use clap::{command, Parser};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
    
    #[arg(short = 'u', long, default_value_t = false)]
    allow_upload: bool,
    #[arg(short = 's', long, default_value_t = false)]
    allow_symlinks: bool,

    path: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("{args:?}");

    let path = match &args.path {
        Some(path) => Path::new(&path).to_owned(),
        None => std::env::current_dir()
            .expect("Can't read current directory")
            .to_owned(),
    };

    nlfmt_serve::run(args.port, &path)
}
