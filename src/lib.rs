mod args;
mod assets;
mod auth;
mod models;
mod qrcode;
mod routes;
mod utils;
mod util;
mod color;

pub use args::ServeArgs;

#[macro_use]
extern crate rocket;

use auth::AuthFairing;
use color::{GREEN, LBLUE, RST, GRAY};
use qrcode::qr_string;
use qrcode_generator::QrCodeEcc;
use rocket::{
    config::Ident,
    data::{Limits, ToByteUnit},
    Config,
};
use rocket_cors::AllowedOrigins;
use util::path::{get_root_dir, pretty_path};

use std::{env, net::IpAddr, path::PathBuf, process::Command};

use models::{AppState, AuthState};
use utils::connection_string;

pub async fn run(args: ServeArgs) -> anyhow::Result<()> {
    let root_dir = get_root_dir(&args.root_dir)?;
    let addr = connection_string(args.interface, args.port);
    let auths = args.auths();
    
    let app_state = AppState {
        root_dir: root_dir.to_path_buf(),
        port: args.port,
        interface: args.interface,
        symlinks: args.symlinks,
        upload: args.upload,
        overwrite: args.overwrite || args.modify,
        delete: args.delete || args.modify,
        rename: args.rename || args.modify,
    };

    if args.qr {
        let matrix =
            qrcode_generator::to_matrix(&addr, QrCodeEcc::Low).expect("Couldn't create QR Code");
        print!("\n{}", qr_string(matrix));
    }

    if args.symlinks {
        println!("\x1b[91mSecurity Warning:\x1b[0m You've enabled symlinks, this can allow users to access arbitrary files on your system. Use with caution.\n")
    }
    
    let mut perms = Vec::new();
    if app_state.symlinks { perms.push("symlinks") }
    if app_state.upload { perms.push("upload") }
    if app_state.overwrite { perms.push("overwrite") }
    if app_state.rename { perms.push("rename") }
    if app_state.delete { perms.push("delete") }

    println!(
        "{GREEN}serve running {RST}@ {LBLUE}{}{RST}\n➜ {GRAY}root: {RST}{}",
        addr,
        pretty_path(&root_dir),
    );

    if perms.len() > 0 {
        println!("➜ {GRAY}enabled: {RST}{}", perms.join(", "))
    }
    println!("");

    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;

    let cfg = get_config(args.interface, args.port);

    let server = rocket::custom(cfg)
        .manage(AuthState { auths })
        .manage(app_state)
        .attach(AuthFairing)
        .attach(cors)
        .mount(
            "/api",
            routes![
                routes::get_dir::get_dir_content,
                routes::download::download_file,
                routes::download_folder::download_folder,
                routes::upload::pre_upload_file,
                routes::upload::upload_file,
                routes::settings::get_settings,
                routes::get_qrcode::get_connection_qrcode,
                routes::file_ops::rename,
                routes::file_ops::delete,
                routes::get_entry_properties::get_entry_properties,
                routes::create_folder::create_folder,
            ],
        )
        .mount("/", routes![routes::get_embedded_file::get_embedded_file])
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

fn get_config(address: IpAddr, port: u16) -> Config {
    Config {
        address,
        port,
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
    }
}
