use rocket::{http::Status, State};
use tokio::fs;

use crate::{auth::AuthGuard, log_error, models::AppState, util::path::parse_relative_path};

#[post("/folder?<path>")]
pub async fn create_folder(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<(), (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => {
            match fs::create_dir_all(path).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    log_error!("Could not create directory: {e}");
                    Err((Status::InternalServerError, "Could not create directory"))
                }
            }
        }
        None => Err((Status::BadRequest, "Invalid path")),
    }
}