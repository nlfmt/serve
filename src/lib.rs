mod assets;
mod handlers;
mod models;
mod utils;

use std::path::Path;

use actix_web::{web, App, HttpServer};
use handlers::{download, index, load_files};
use models::AppState;
use utils::pretty_path;

#[actix_web::main]
pub async fn run(port: u16, path: &Path) -> std::io::Result<()> {
    let path = path.canonicalize()?;

    println!(
        "serving \x1b[33m{}\x1b[0m on port \x1b[33m{}\x1b[0m",
        pretty_path(&path),
        port
    );

    let state = web::Data::new(AppState { file_dir: path.to_path_buf() });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(load_files)
            .service(download)
            .default_service(web::get().to(index))
    })
    .bind(("localhost", port))?
    .run()
    .await
}
