use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use actix_files::NamedFile;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::{arg, Parser};
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use serve::pretty_path;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    path: Option<String>,
}

struct AppState {
    file_dir: PathBuf,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(Assets::get("index.html").unwrap().data)
}

#[derive(Serialize)]
struct FilesResponse {
    files: Vec<String>,
    dirs: Vec<String>,
}
#[derive(Deserialize)]
struct FilesQuery {
    path: String,
}

#[get("/api/files")]
async fn load_files(
    data: web::Data<AppState>,
    params: web::Query<FilesQuery>,
) -> Result<impl Responder, std::io::Error> {
    let path = data.file_dir.join(params.path.clone()).canonicalize()?;

    if !path.starts_with(&data.file_dir) {
        return Ok(HttpResponse::BadRequest().body("Path is not valid"));
    }

    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();

        if entry.metadata().unwrap().is_dir() {
            dirs.push(file_name);
        } else {
            files.push(file_name);
        }
    }

    Ok(HttpResponse::Ok().json(FilesResponse { files, dirs }))
}

#[derive(Deserialize)]
struct DownloadQuery {
    path: String,
}

#[get("/api/download")]
async fn download(
    data: web::Data<AppState>,
    params: web::Query<DownloadQuery>,
) -> std::io::Result<NamedFile> {
    let path = data.file_dir.join(params.path.clone()).canonicalize()?;

    if !path.starts_with(&data.file_dir) {
        return Err(std::io::Error::new(
            ErrorKind::PermissionDenied,
            "You can't access this path",
        ));
    }

    println!(
        "download \x1b[33m{}\x1b[0m from \x1b[33m{}\x1b[0m",
        path.file_name().unwrap().to_str().unwrap(),
        path.parent().unwrap().display()
    );

    Ok(NamedFile::open(path).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let path = match &args.path {
        Some(path) => Path::new(&path).to_owned(),
        None => std::env::current_dir()
            .expect("Can't read current directory")
            .to_owned(),
    }
    .canonicalize()?;
    let state = web::Data::new(AppState {
        file_dir: path.clone(),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(load_files)
            .service(download)
            .default_service(web::get().to(index))
    })
    .bind(("localhost", args.port))?
    .run();

    println!(
        "serving \x1b[33m{}\x1b[0m on port \x1b[33m{}\x1b[0m",
        pretty_path(&path),
        args.port
    );

    server.await
}
