use std::path::{Path, PathBuf};

use rocket::{data::ToByteUnit, http::Status, State};
use tokio::fs;

use crate::{auth::AuthGuard, models::{AppState, UploadQuery}, util::path::parse_relative_path};

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

#[head("/upload?<query..>")]
pub async fn pre_upload_file(_auth: AuthGuard, state: &State<AppState>, query: UploadQuery) -> Status {
    if !state.upload {
        return Status::Forbidden;
    }

    match validate_upload_path(
        &state.root_dir,
        &query.path,
        &query.file_name,
        query.overwrite,
        state.symlinks,
    ) {
        Ok(_) => Status::Ok,
        Err((status, _)) => Status::from_code(status).unwrap(),
    }
}

#[post("/upload?<query..>", data = "<data>")]
pub async fn upload_file(
    _auth: AuthGuard,
    state: &State<AppState>,
    query: UploadQuery,
    data: rocket::Data<'_>,
) -> Result<(), (Status, String)> {
    if !state.upload {
        return Err((Status::Forbidden, "File Uploads are disabled".to_string()));
    }

    match validate_upload_path(
        &state.root_dir,
        &query.path,
        &query.file_name,
        query.overwrite,
        state.symlinks,
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