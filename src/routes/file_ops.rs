use std::fs;

use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;

use crate::{auth::AuthGuard, log_error, models::AppState, util::path::{parse_relative_path, pretty_path}};
use crate::color::{ORANGE, LBLUE};

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
        Some(path) => match fs::rename(&path, &data.to) {
            Ok(_) => {
                println!("{ORANGE}rename {LBLUE}{} \x1b[0m-> {LBLUE}{}\x1b[0m", pretty_path(&path), data.to);
                Ok(())
            }
            Err(e) => {
                log_error!("Could not rename item: {e}");
                Err((Status::InternalServerError, "Could not rename item"))
            }
        }
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
        Some(path) => {
            let res = match path.metadata().unwrap().is_dir() {
                true => (fs::remove_dir_all(&path), "folder"),
                false => (fs::remove_file(&path), "file"),
            };
            match res {
                (Ok(_), t) => {
                    println!("{ORANGE}delete\x1b[0m {} {LBLUE}{}\x1b[0m", t, pretty_path(&path));
                    Ok(())
                }
                (Err(e), t) => {
                    log_error!("Could not delete {}: {e}", t);
                    Err((Status::InternalServerError, "Could not delete item"))
                }
            }
        }
        None => Err((Status::BadRequest, "Invalid path")),
    }
}