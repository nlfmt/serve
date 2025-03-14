mod args;
mod assets;
mod models;
mod qrcode;
mod routes;
mod utils;

pub use args::ServeArgs;

#[macro_use]
extern crate rocket;

use qrcode::qr_string;
use qrcode_generator::QrCodeEcc;
use rocket::{
    config::Ident,
    data::{Limits, ToByteUnit},
    Config,
};
use rocket_cors::AllowedOrigins;

use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use models::AppState;
use utils::{connection_string, pretty_path};

pub async fn run(args: ServeArgs) -> anyhow::Result<()> {
    let root_dir = match &args.root_dir {
        Some(path) => Path::new(&path).to_owned(),
        None => std::env::current_dir()
            .expect("Can't read current directory")
            .to_owned(),
    }
    .canonicalize()?;

    let addr = connection_string(
        args.interface,
        if cfg!(debug_assertions) {
            3001
        } else {
            args.port
        },
    );

    if args.qr {
        let matrix =
            qrcode_generator::to_matrix(&addr, QrCodeEcc::Low).expect("Couldn't create QR Code");
        print!("\n{}", qr_string(matrix));
    }

    if args.allow_symlinks {
        println!("\x1b[91mSecurity Warning:\x1b[0m You've enabled symlinks, this can allow users to access arbitrary files on your system. Use with caution.")
    }

    println!(
        "serving \x1b[33m{}\x1b[0m on \x1b[33m{}\x1b[0m",
        pretty_path(&root_dir),
        addr
    );

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;

    let cfg = Config {
        address: args.interface,
        port: args.port,
        ident: Ident::try_new("serve").unwrap(),

        limits: Limits::default()
            .limit("data-form", 10.gigabytes())
            .limit("file", 10.gigabytes()),

        log_level: if cfg!(debug_assertions) {
            rocket::config::LogLevel::Normal
        } else {
            rocket::config::LogLevel::Off
        },
        ..Default::default()
    };

    let server = rocket::custom(cfg)
        .manage(AppState {
            root_dir: root_dir.to_path_buf(),
            port: args.port,
            interface: args.interface,
            allow_symlinks: args.allow_symlinks,
            allow_upload: args.allow_upload,
        })
        .mount(
            "/api",
            routes![
                routes::get_dir::get_dir_content,
                routes::download::download_file,
                routes::upload::pre_upload_file,
                routes::upload::upload_file,
                routes::settings::get_upload_enabled,
                routes::get_qrcode::get_connection_qrcode,
            ],
        )
        .mount("/", routes![routes::get_embedded_file::get_embedded_file])
        .attach(cors)
        .launch();

    // run vite dev server in debug mode
    if cfg!(debug_assertions) {
        let project_root =
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

        std::env::set_current_dir(&project_root).unwrap();
        Command::new("powershell")
            .arg("-c")
            .arg("pnpm dev")
            .current_dir("./app")
            .spawn()
            .unwrap();
    }

    server.await?;
    Ok(())
}
