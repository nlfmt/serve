pub mod assets;
mod handlers;
mod models;
pub mod utils;

use std::{path::Path, process::Command};

use actix_web::{web, App, HttpServer};
use handlers::{download_file, load_files, serve_embedded_file, upload_file};
use models::AppState;
use utils::pretty_path;

#[actix_web::main]
pub async fn run(port: u16, path: &Path) -> anyhow::Result<()> {
    let path = path.canonicalize()?;

    println!(
        "serving \x1b[33m{}\x1b[0m on port \x1b[33m{}\x1b[0m",
        pretty_path(&path),
        port
    );

    let state = web::Data::new(AppState { file_dir: path.to_path_buf() });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(load_files)
            .service(download_file)
            .service(upload_file)
            .default_service(web::get().to(serve_embedded_file))
    })
    .bind(("0.0.0.0", port))?
    .run();
    
    // run vite dev server in debug mode
    if cfg!(debug_assertions) {
        Command::new("powershell")
            .arg("-c")
            .arg("pnpm dev")
            .current_dir(std::env::current_dir().unwrap().join("./app"))
            .spawn()
            .unwrap();
    }
    
    Ok(server.await?)
}


// create a function that says hello
