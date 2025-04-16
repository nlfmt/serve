mod args;
mod assets;
mod auth;
mod color;
mod models;
mod qrcode;
mod routes;
mod state;
mod updater;
mod util;
mod utils;
mod server;

pub use args::ServeArgs;

#[macro_use]
extern crate rocket;

use color::{GRAY, GREEN, LBLUE, RST};
use qrcode::qr_string;
use qrcode_generator::QrCodeEcc;
use server::launch_server;
use state::AppState;
use util::path::{get_root_dir, pretty_path};
use utils::connection_string;

pub async fn run(args: ServeArgs) -> anyhow::Result<()> {
    if args.update {
        return updater::run_update().await.map_err(anyhow::Error::msg);
    }

    updater::run_background_check();

    let root_dir = get_root_dir(&args.root_dir)?;
    let addr = connection_string(args.interface, args.port);
    let app_state = AppState::new(&args, &root_dir);

    if args.qr {
        let matrix =
            qrcode_generator::to_matrix(&addr, QrCodeEcc::Low).expect("Couldn't create QR Code");
        print!("\n{}", qr_string(matrix));
    }

    if args.symlinks {
        println!("\x1b[91mSecurity Warning:\x1b[0m You've enabled symlinks, this can allow users to access arbitrary files on your system. Use with caution.\n")
    }

    println!(
        "{GREEN}serve running {RST}@ {LBLUE}{}{RST}\n➜ {GRAY}root: {RST}{}",
        addr,
        pretty_path(&root_dir),
    );

    let perms = app_state.get_perms();
    if perms.len() > 0 {
        println!("➜ {GRAY}enabled: {RST}{}", perms.join(", "))
    }
    println!("");

    // run vite dev server in debug mode
    #[cfg(debug_assertions)]
    launch_dev_server();

    launch_server(args, app_state).await?;
    Ok(())
}

#[cfg(debug_assertions)]
fn launch_dev_server() {
    use std::{path::PathBuf, process::Command};

    let project_root =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    std::env::set_current_dir(&project_root).unwrap();
    Command::new("powershell")
        .arg("-c")
        .arg("pnpm dev")
        .current_dir("./app")
        .spawn()
        .unwrap();
}
