use std::io::ErrorKind;

use rocket::{http::Status, serde::json::Json, State, response::Responder};

use crate::{auth::AuthGuard, log_error, models::FilesQuery, state::AppState, util::{dir::read_entries, path::parse_relative_path}};

#[get("/files?<query..>")]
pub async fn get_dir_content(_auth: AuthGuard, state: &State<AppState>, query: FilesQuery) -> impl Responder {
    match parse_relative_path(&state.root_dir, &query.path, state.symlinks) {
        None => Err((Status::BadRequest, "Invalid path")),
        Some(path) => match read_entries(&path, state.symlinks) {
            Ok(content) => Ok(Json(content)),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err((Status::NotFound, "Directory not found")),
                _ => {
                    log_error!("Could not read directory: {e}");
                    Err((Status::InternalServerError, "Could not read directory"))
                },
            },
        },
    }
}