mod assets;
mod handlers;
mod models;
mod utils;

#[macro_use]
extern crate rocket;

pub use models::ServeOptions;
use rocket::{config::Ident, data::{Limits, ToByteUnit}, Config};
use rocket_cors::AllowedOrigins;

use std::{env, net::IpAddr, path::PathBuf, process::Command, str::FromStr};

use handlers::{download_file, get_upload_enabled, load_files, pre_upload_file, serve_embedded_file, upload_file};
use models::AppState;
use utils::pretty_path;

pub async fn run(opts: ServeOptions<'_>) -> anyhow::Result<()> {
    let path = opts.path.canonicalize()?;

    println!(
        "serving \x1b[33m{}\x1b[0m on port \x1b[33m{}\x1b[0m",
        pretty_path(&path),
        opts.port
    );
    
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;
    
    let cfg = Config {
        address: IpAddr::from_str("0.0.0.0").unwrap(),
        port: opts.port,
        ident: Ident::try_new("serve").unwrap(),
        limits: Limits::default()
            .limit("data-form", 10.gigabytes())
            .limit("file", 10.gigabytes()),
        ..Default::default()
    };

    let server = rocket::custom(cfg)
        .manage(AppState {
            file_dir: path.to_path_buf(),
            allow_symlinks: opts.allow_symlinks,
            allow_upload: opts.allow_upload,
        })
        .mount("/", routes![
            load_files,
            download_file,
            pre_upload_file,
            upload_file,
            get_upload_enabled,
            serve_embedded_file
        ])
        .attach(cors)
        .launch();

    // run vite dev server in debug mode
    if cfg!(debug_assertions) {
        Command::new("powershell")
            .arg("-c")
            .arg("pnpm dev")
            .current_dir(
                PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
                    .join("./app"),
            )
            .spawn()
            .unwrap();
    }

    server.await?;
    Ok(())
}
