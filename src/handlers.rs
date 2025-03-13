use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    str::FromStr,
};

use mime_guess::from_path;
use rocket::{
    data::ToByteUnit,
    fs::NamedFile,
    http::{ContentType, Header, Status},
    response::{self, Responder},
    serde::json::Json,
    Data, Response, State,
};
use tokio::fs;

use crate::{
    assets::Assets,
    models::{AppState, DownloadQuery, FilesQuery, UploadQuery},
    utils::{parse_relative_path, pretty_path, read_entries},
};

#[get("/api/files?<query..>")]
pub async fn load_files(state: &State<AppState>, query: FilesQuery) -> impl Responder {
    match parse_relative_path(&state.file_dir, &query.path, state.allow_symlinks) {
        None => Err((Status::BadRequest, "Invalid path")),
        Some(path) => match read_entries(&path, state.allow_symlinks) {
            Ok(content) => Ok(Json(content)),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err((Status::NotFound, "Directory not found")),
                _ => Err((Status::InternalServerError, "Could not read directory")),
            },
        },
    }
}

#[get("/api/download?<query..>")]
pub async fn download_file(
    state: &State<AppState>,
    query: DownloadQuery,
) -> Result<FileResponse, (Status, &str)> {
    println!("{query:?}");
    match parse_relative_path(&state.file_dir, &query.path, state.allow_symlinks) {
        Some(path) => {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            println!(
                "download \x1b[33m{}\x1b[0m from \x1b[33m{}\x1b[0m",
                &file_name,
                pretty_path(path.parent().unwrap())
            );
            let file = NamedFile::open(&path).await.unwrap();
            Ok(FileResponse {
                inner: file,
                file_name,
            })
        }
        None => Err((Status::BadRequest, "Invalid path")),
    }
}

fn validate_upload_path(
    root: &Path,
    path: &str,
    file_name: &str,
    overwrite: bool,
    allow_symlinks: bool,
) -> Result<PathBuf, (u16, &'static str)> {
    let file_path = Path::new(path)
        .join(file_name)
        .to_str()
        .and_then(|file_path| parse_relative_path(root, file_path, allow_symlinks));

    match file_path {
        Some(path) => match path.exists() && !overwrite {
            true => Err((409, "File already exists")),
            false => Ok(path),
        },
        None => Err((400, "Invalid Path")),
    }
}

#[head("/api/upload?<query..>")]
pub async fn pre_upload_file(state: &State<AppState>, query: UploadQuery) -> Status {
    if !state.allow_upload {
        return Status::Forbidden;
    }

    match validate_upload_path(
        &state.file_dir,
        &query.path,
        &query.file_name,
        query.overwrite,
        state.allow_symlinks,
    ) {
        Ok(_) => Status::Ok,
        Err((status, _)) => Status::from_code(status).unwrap(),
    }
}

#[post("/api/upload?<query..>", data = "<data>")]
pub async fn upload_file<'a>(
    state: &State<AppState>,
    query: UploadQuery,
    data: Data<'_>,
) -> Result<(), (Status, String)> {
    if !state.allow_upload {
        return Err((Status::Forbidden, "File Uploads are disabled".to_string()));
    }

    match validate_upload_path(
        &state.file_dir,
        &query.path,
        &query.file_name,
        query.overwrite,
        state.allow_symlinks,
    ) {
        Ok(path) => {
            if let Some(parent_dir) = path.parent() {
                let _ = fs::create_dir_all(parent_dir).await;
            };

            let file = tokio::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .await
                .map_err(|e| {
                    (
                        Status::InternalServerError,
                        format!("Failed to open file: {}", e.to_string()).to_string(),
                    )
                })?;

            data.open(10.gigabytes())
                .stream_to(file)
                .await
                .map_err(|e| {
                    (
                        Status::InternalServerError,
                        format!("Failed to write to file: {}", e.to_string()).to_string(),
                    )
                })?;
            Ok(())
        }
        Err((status, message)) => Err((Status::from_code(status).unwrap(), message.to_string())),
    }
}

#[get("/<path..>")]
pub async fn serve_embedded_file(
    path: PathBuf,
) -> Result<(ContentType, Vec<u8>), (Status, &'static str)> {
    let path = path.to_str().unwrap();
    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(content) => {
            let mime_type = from_path(path).first_or_octet_stream();
            Ok((
                ContentType::from_str(&mime_type.to_string()).unwrap(),
                content.data.to_vec(),
            ))
        }
        None => Err((Status::NotFound, "Not Found")),
    }
}

#[get("/api/upload_enabled")]
pub async fn get_upload_enabled(state: &State<AppState>) -> String {
    state.allow_upload.to_string()
}
