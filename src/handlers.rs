use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{
    get, head,
    http::{header::CONTENT_LENGTH, StatusCode},
    post, web, HttpRequest, HttpResponse, Responder,
};
use futures_util::TryStreamExt;
use mime_guess::from_path;
use tokio::{fs, io::AsyncWriteExt};

use crate::{
    assets::Assets,
    models::{AppState, DownloadQuery, FilesQuery, UploadQuery},
    utils::{parse_relative_path, pretty_path, read_entries},
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
                "downloaded \x1b[33m{}\x1b[0m from \x1b[33m{}\x1b[0m",
                path.file_name().unwrap().to_str().unwrap(),
                pretty_path(path.parent().unwrap())
            );
            Ok(NamedFile::open(path).unwrap())
        }
        None => Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid path")),
    }
}

fn validate_upload_path(
    root: &Path,
    path: &str,
    file_name: &str,
    overwrite: bool,
) -> Result<PathBuf, (u16, String)> {
    let file_path = Path::new(path)
        .join(file_name)
        .to_str()
        .and_then(|file_path| parse_relative_path(root, file_path));

    match file_path {
        Some(path) => match path.exists() && !overwrite {
            true => Err((409, "File already exists".to_string())),
            false => Ok(path),
        },
        None => Err((400, "Invalid Path".to_string())),
    }
}

#[head("/api/upload")]
pub async fn pre_upload_file(
    data: web::Data<AppState>,
    params: web::Query<UploadQuery>,
) -> impl Responder {
    if !data.allow_upload {
        return HttpResponse::Forbidden().finish();
    }

    match validate_upload_path(
        &data.file_dir,
        &params.path,
        &params.file_name,
        params.overwrite,
    ) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err((status, _)) => HttpResponse::build(StatusCode::from_u16(status).unwrap()).finish(),
    }
}

#[post("/api/upload")]
pub async fn upload_file(
    data: web::Data<AppState>,
    mut payload: Multipart,
    req: HttpRequest,
    params: web::Query<UploadQuery>,
) -> impl Responder {
    if !data.allow_upload {
        return HttpResponse::Forbidden().body("File Uploads are disabled");
    }

    match validate_upload_path(
        &data.file_dir,
        &params.path,
        &params.file_name,
        params.overwrite,
    ) {
        Ok(path) => {
            let max_file_size: usize = 10_000_000_000;
            let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
                Some(len) => len.to_str().unwrap_or("0").parse().unwrap_or(0),
                None => 0,
            };

            if content_length == 0 || content_length > max_file_size {
                return HttpResponse::BadRequest().body("File too big, max size is 10GB");
            }

            while let Ok(Some(mut field)) = payload.try_next().await {
                if !field.name().is_some_and(|f| f == "file") {
                    continue;
                }
                if let Some(parent_dir) = path.parent() {
                    let _ = fs::create_dir_all(parent_dir).await;
                };
                let mut file = match fs::File::create(&path).await {
                    Ok(f) => f,
                    Err(_) => {
                        return HttpResponse::InternalServerError().body("Could not create file")
                    }
                };
                while let Ok(Some(chunk)) = field.try_next().await {
                    let _ = file.write_all(&chunk).await.unwrap();
                }
            }
            println!(
                "uploaded \x1b[33m{}\x1b[0m to \x1b[33m{}\x1b[0m",
                params.file_name, params.path
            );
            HttpResponse::Created().finish()
        }
        Err((status, message)) => {
            HttpResponse::build(StatusCode::from_u16(status).unwrap()).body(message)
        }
    }
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
#[get("/api/upload_enabled")]
pub async fn get_upload_enabled(data: web::Data<AppState>) -> String {
    return data.allow_upload.to_string();
}
