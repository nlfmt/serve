mod assets;
mod handlers;
mod models;
mod utils;

pub use models::ServeOptions;

use std::{env, path::PathBuf, process::Command};

use actix_web::{web, App, HttpServer};
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

    let state = web::Data::new(AppState {
        file_dir: path.to_path_buf(),
        allow_symlinks: opts.allow_symlinks,
        allow_upload: opts.allow_upload,
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(load_files)
            .service(download_file)
            .service(pre_upload_file)
            .service(upload_file)
            .service(get_upload_enabled)
            .default_service(web::get().to(serve_embedded_file))
    })
    .keep_alive(None)
    .bind(("0.0.0.0", opts.port))?
    .run();

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

    Ok(server.await?)
}
