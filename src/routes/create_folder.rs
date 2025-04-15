use rocket::{http::Status, State};
use tokio::fs;

use crate::color::{GREEN, LBLUE};
use crate::{
    auth::AuthGuard,
    log_error,
    models::AppState,
    util::path::{parse_relative_path, pretty_path},
};

#[post("/folder?<path>")]
pub async fn create_folder(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<(), (Status, &str)> {
    if !state.upload {
        return Err((Status::Forbidden, "Creating folders is not enabled"));
    }

    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => match fs::create_dir_all(&path).await {
            Ok(_) => {
                println!("{GREEN}create folder {LBLUE}{}", pretty_path(&path));
                Ok(())
            }
            Err(e) => {
                log_error!("Could not create directory: {e}");
                Err((Status::InternalServerError, "Could not create directory"))
            }
        },
        None => Err((Status::BadRequest, "Invalid path")),
    }
}
