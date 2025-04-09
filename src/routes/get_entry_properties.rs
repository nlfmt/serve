use std::time::UNIX_EPOCH;

use rocket::{http::Status, serde::json::Json, State};

use crate::{
    log_error, models::{AppState, EntryProperties}, util::path::parse_relative_path
};

#[get("/properties?<path>")]
pub fn get_entry_properties(
    state: &State<AppState>,
    path: String,
) -> Result<Json<EntryProperties>, (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => {
            let meta = match path.metadata() {
                Err(e) => {
                    log_error!("Could not get entry metadata: {e}");
                    return Err((
                        Status::InternalServerError,
                        "Could not get entry metadata",
                    ))
                }
                Ok(v) => v,
            };
            let accessed = meta
                .accessed()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));
            let readonly = meta.permissions().readonly();

            Ok(Json(EntryProperties {
                accessed,
                readonly
            }))
        }
        None => Err((Status::BadRequest, "Invalid path")),
    }
}
