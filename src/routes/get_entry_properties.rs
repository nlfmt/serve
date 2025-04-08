use std::time::UNIX_EPOCH;

use rocket::{http::Status, serde::json::Json, State};

use crate::{
    models::{AppState, EntryProperties},
    util::path::parse_relative_path,
};

#[get("/properties?<path>")]
pub fn get_entry_properties(
    state: &State<AppState>,
    path: String,
) -> Result<Json<EntryProperties>, (Status, String)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => {
            let meta = match path.metadata() {
                Err(e) => {
                    return Err((
                        Status::InternalServerError,
                        format!("Could not get entry metadata: {e}"),
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
        None => Err((Status::BadRequest, "Invalid path".to_string())),
    }
}
