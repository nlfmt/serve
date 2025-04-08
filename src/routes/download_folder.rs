use std::{fs::File, path::Path};

use rocket::{fs::NamedFile, http::Status, State};
use walkdir::WalkDir;

use crate::{auth::AuthGuard, models::{AppState, FileResponse}, util::{path::{generate_temp_path, parse_relative_path}, zip::zip_dir}};

#[get("/download_folder?<path>")]
pub async fn download_folder(
    _auth: AuthGuard,
    state: &State<AppState>,
    path: String,
) -> Result<FileResponse, (Status, &str)> {
    match parse_relative_path(&state.root_dir, &path, state.symlinks) {
        Some(path) => {
            let tmp_file_path = generate_temp_path();
            let tmp_path = Path::new(&tmp_file_path);
            let tmp_file = File::create(tmp_path).unwrap();

            let walkdir = WalkDir::new(&path).follow_links(false);
            let it = walkdir.into_iter();

            match zip_dir(&mut it.filter_map(|e| e.ok()), &path, tmp_file) {
                Ok(_) => {
                    let file = NamedFile::open(tmp_path).await.unwrap();
                    let file_name = format!("{}.zip", path.file_name().unwrap().to_str().unwrap());
                    Ok(FileResponse { inner: file, file_name })
                }
                Err(e) => {
                    Err((Status::InternalServerError, "Could not create zip file"))
                }
            }
        }
        None => Err((Status::BadRequest, "Invalid Path"))
    }
}
