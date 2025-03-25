use std::io::ErrorKind;

use rocket::{http::Status, serde::json::Json, State, response::Responder};

use crate::{auth::AuthGuard, models::{AppState, FilesQuery}, util::{dir::read_entries, path::parse_relative_path}};

#[get("/files?<query..>")]
pub async fn get_dir_content(_auth: AuthGuard, state: &State<AppState>, query: FilesQuery) -> impl Responder {
    match parse_relative_path(&state.root_dir, &query.path, state.allow_symlinks) {
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