use std::fs;

use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;

use crate::{auth::AuthGuard, models::AppState, util::path::parse_relative_path};

#[derive(Deserialize)]
pub struct RenameQuery {
    path: String,
    to: String,
}

#[put("/rename", data = "<data>")]
pub fn rename(
    _auth: AuthGuard,
    state: &State<AppState>,
    data: Json<RenameQuery>,
) -> Result<(), (Status, &str)> {
    let data = data.0;
    match parse_relative_path(&state.root_dir, &data.path, state.symlinks) {
        Some(path) => fs::rename(path, data.to)
            .map_err(|_e| (Status::InternalServerError, "Could not rename item")),
        None => Err((Status::BadRequest, "Invalid path")),
    }
}

#[delete("/delete?<path>")]
pub fn delete(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<(), (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => match path.metadata().unwrap().is_dir() {
            true => fs::remove_dir_all(path),
            false => fs::remove_file(path),
        }.map_err(|_| (Status::InternalServerError, "Could not delete item")),
        None => Err((Status::BadRequest, "Invalid path")),
    }
}