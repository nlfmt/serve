use clap::{command, Parser};
use nlfmt_serve::{run, ServeOptions};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ServeArgs {
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    
    #[arg(short = 'u', long, default_value_t = false)]
    pub allow_upload: bool,
    #[arg(short = 's', long, default_value_t = false)]
    pub allow_symlinks: bool,

    pub path: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = ServeArgs::parse();

    let path = match &args.path {
        Some(path) => Path::new(&path).to_owned(),
        None => std::env::current_dir()
            .expect("Can't read current directory")
            .to_owned(),
    };

    run(ServeOptions {
        path: &path,
        port: args.port,
        allow_symlinks: args.allow_symlinks,
        allow_upload: args.allow_upload,
    }).await
}
