use std::{
    fs::File,
    io::{ErrorKind, Write},
    path::Path,
};

use actix_files::NamedFile;
use actix_multipart::{form::MultipartForm, Field, Multipart};
use actix_web::{
    get, http::header::CONTENT_LENGTH, post, web, HttpRequest, HttpResponse, Responder,
};
use futures_util::TryStreamExt;
use mime_guess::from_path;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    assets::Assets,
    models::{AppState, DownloadQuery, FilesQuery, UploadQuery},
    utils::{parse_relative_path, parse_string_field, pretty_path, read_entries},
};

#[get("/api/files")]
pub async fn load_files(data: web::Data<AppState>, params: web::Query<FilesQuery>) -> HttpResponse {
    match parse_relative_path(&data.file_dir, &params.path) {
        None => HttpResponse::BadRequest().body("Invalid path"),
        Some(path) => match read_entries(&path) {
            Ok(content) => HttpResponse::Ok().json(content),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => HttpResponse::NotFound().body("Directory not found"),
                _ => HttpResponse::InternalServerError().body("Could not read directory"),
            },
        },
    }
}

#[get("/api/download")]
pub async fn download_file(
    data: web::Data<AppState>,
    params: web::Query<DownloadQuery>,
) -> std::io::Result<NamedFile> {
    match parse_relative_path(&data.file_dir, &params.path) {
        Some(path) => {
            println!(
                "download \x1b[33m{}\x1b[0m from \x1b[33m{}\x1b[0m",
                path.file_name().unwrap().to_str().unwrap(),
                pretty_path(path.parent().unwrap())
            );
            Ok(NamedFile::open(path).unwrap())
        }
        None => Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid path")),
    }
}

#[post("/api/upload")]
pub async fn upload_file(
    data: web::Data<AppState>,
    mut payload: Multipart,
    req: HttpRequest,
    params: web::Query<UploadQuery>,
) -> impl Responder {
    let file_path = Path::new(params.path.as_str()).join(params.file_name.as_str());
    println!("{params:?}");

    match file_path.to_str().and_then(|file_path| parse_relative_path(&data.file_dir, file_path)) {
        Some(path) => {
            if path.exists() && !params.overwrite {
                return HttpResponse::Conflict().body("File already exists");
            }

            let max_file_size: usize = 10_000_000_000;
            let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
                Some(len) => len.to_str().unwrap_or("0").parse().unwrap_or(0),
                None => 0,
            };

            if content_length == 0 || content_length > max_file_size {
                return HttpResponse::BadRequest().body("File too big, max size is 10GB");
            }
            println!("parsing fields");
            while let Ok(Some(mut field)) = payload.try_next().await {
                if !field.name().is_some_and(|f| f == "file") {
                    continue;
                }
                println!("found file field");
                if let Some(parent_dir) = path.parent() {
                    let _ = fs::create_dir_all(parent_dir).await;
                };
                let mut file = fs::File::create(&path).await.unwrap();
                println!("streaming file");
                while let Ok(Some(chunk)) = field.try_next().await {
                    let _ = file.write_all(&chunk).await.unwrap();
                }
            }
            println!("done");
            HttpResponse::Created().finish()
        }
        None => HttpResponse::BadRequest().body("Invalid Path")
    }

    // match file_path
    //     .to_str()
    //     .and_then(|path| parse_relative_path(&data.file_dir, path))
    // {
    //     Some(path) => {
    //         println!("checking path");
    //         if path.exists() && form.overwrite.to_string() != "true" {
    //             return HttpResponse::Conflict().body("File exists");
    //         }

    //         // match File::create(&path) {
    //         //     Ok(mut file) => {
    //         //         match file.write(&form.file.data) {
    //         //             Ok(_) => HttpResponse::Created().finish(),
    //         //             Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    //         //         }
    //         //     },
    //         //     Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    //         // }
    //         HttpResponse::Ok().finish()
    //     }
    //     None => HttpResponse::BadRequest().body("Invalid Path"),
    // }
}

pub async fn serve_embedded_file(req: HttpRequest) -> impl Responder {
    let path = req.path().trim_start_matches('/');

    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(content) => {
            let mime_type = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime_type.to_string())
                .body(content.data.to_vec())
        }
        None => HttpResponse::NotFound().body("404 - Not Found"),
    }
}
