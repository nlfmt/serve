use std::path::{Path, PathBuf};

use rocket::{data::ToByteUnit, http::Status, State};
use tokio::fs;

use crate::{auth::AuthGuard, log_error, models::{AppState, UploadQuery}, util::path::parse_relative_path};

fn validate_upload_path(
    path: &str,
    file_name: &str,
    overwrite: bool,
    state: &State<AppState>
) -> Result<PathBuf, (Status, &'static str)> {
    if overwrite && !state.overwrite {
        return Err((Status::Forbidden, "Overwriting files is not allowed"))
    }
    let file_path = Path::new(path)
        .join(file_name)
        .to_str()
        .and_then(|file_path| parse_relative_path(&state.root_dir, file_path, state.symlinks));

    match file_path {
        Some(path) => match path.exists() && !overwrite {
            true => Err((Status::Conflict, "File already exists")),
            false => Ok(path),
        },
        None => Err((Status::BadRequest, "Invalid Path")),
    }
}

#[head("/upload?<query..>")]
pub async fn pre_upload_file(_auth: AuthGuard, state: &State<AppState>, query: UploadQuery) -> Status {
    if !state.upload {
        return Status::Forbidden;
    }

    match validate_upload_path(
        &query.path,
        &query.file_name,
        query.overwrite,
        state,
    ) {
        Ok(_) => Status::Ok,
        Err((status, _)) => status,
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
        &query.path,
        &query.file_name,
        query.overwrite,
        state,
    ) {
        Ok(path) => {
            if let Some(parent_dir) = path.parent() {
                let _ = fs::create_dir_all(parent_dir).await;
            };

            let file = tokio::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .await
                .map_err(|e| {
                    log_error!("Failed to open file: {e}");
                    (
                        Status::InternalServerError,
                        "Failed to open file".to_string(),
                    )
                })?;

            data.open(10.gigabytes())
                .stream_to(file)
                .await
                .map_err(|_e| {
                    if let Err(e) = std::fs::remove_file(&path) {
                        log_error!("Failed to clean up temporary upload file '{}': {e}", path.display());
                    }
                    (
                        Status::InternalServerError,
                        "Upload Failed".to_string(),
                    )
                })?;

            println!(
                "upload \x1b[33m{}\x1b[0m to \x1b[33m{}\x1b[0m",
                &query.file_name,
                &query.path
            );
            Ok(())
        }
        Err((status, message)) => Err((status, message.to_string())),
    }
}