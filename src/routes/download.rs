use rocket::{fs::NamedFile, http::Status, State};

use crate::{auth::AuthGuard, models::FileResponse, state::AppState, util::path::{parse_relative_path, pretty_path}};
use crate::color::{BLUE, LBLUE};

#[get("/download?<path>")]
pub async fn download_file(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<FileResponse, (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            println!(
                "{BLUE}download {LBLUE}{}\x1b[0m",
                pretty_path(&path)
            );
            let file = NamedFile::open(&path).await.unwrap();
            Ok(FileResponse {
                inner: file,
                file_name,
            })
        }
        None => Err((Status::BadRequest, "Invalid path")),
    }
}