use std::io::ErrorKind;

use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    assets::Assets,
    models::{AppState, DownloadQuery, FilesQuery},
    utils::{parse_relative_path, pretty_path, read_entries},
};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(Assets::get("index.html").unwrap().data)
}

#[get("/api/files")]
pub async fn load_files(data: web::Data<AppState>, params: web::Query<FilesQuery>) -> HttpResponse {
    match parse_relative_path(&data.file_dir, &params.path) {
        Err(e) => HttpResponse::BadRequest().body(e),
        Ok(path) => match read_entries(&path) {
            Ok(content) => HttpResponse::Ok().json(content),
            Err(_) => HttpResponse::InternalServerError().body("Could not read directory"),
        },
    }
}

#[get("/api/download")]
pub async fn download(
    data: web::Data<AppState>,
    params: web::Query<DownloadQuery>,
) -> std::io::Result<NamedFile> {
    match parse_relative_path(&data.file_dir, &params.path) {
        Ok(path) => {
            println!(
                "download \x1b[33m{}\x1b[0m from \x1b[33m{}\x1b[0m",
                path.file_name().unwrap().to_str().unwrap(),
                pretty_path(path.parent().unwrap())
            );
            Ok(NamedFile::open(path).unwrap())
        }
        Err(e) => Err(std::io::Error::new(ErrorKind::PermissionDenied, e)),
    }
}
