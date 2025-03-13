use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, http::Header, response, serde::Serialize, Response};

pub struct ServeOptions<'a> {
    pub path: &'a Path,
    pub port: u16,
    pub allow_upload: bool,
    pub allow_symlinks: bool,
}

pub struct AppState {
    pub file_dir: PathBuf,
    pub allow_upload: bool,
    pub allow_symlinks: bool,
}

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub modified: Option<u64>,
    pub created: Option<u64>,
    pub is_symlink: bool,
}

#[derive(Serialize)]
pub struct FolderInfo {
    pub name: String,
    pub modified: Option<u64>,
    pub created: Option<u64>,
    pub is_symlink: bool,
}

#[derive(Serialize)]
pub struct DirectoryContent {
    pub dirs: Vec<FolderInfo>,
    pub files: Vec<FileInfo>,
}
impl Default for DirectoryContent {
    fn default() -> Self {
        DirectoryContent {
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }
}

#[derive(FromForm)]
pub struct FilesQuery {
    pub path: String,
}

#[derive(FromForm, Debug)]
pub struct DownloadQuery {
    pub path: String,
}

#[derive(FromForm)]
pub struct UploadQuery {
    pub path: String,
    pub file_name: String,
    pub overwrite: bool,
}

pub struct FileResponse {
    inner: NamedFile,
    file_name: String,
}
impl<'r> rocket::response::Responder<'r, 'r> for FileResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> response::Result<'r> {
        Response::build()
            .header(Header::new(
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", self.file_name),
            ))
            .streamed_body(self.inner.take_file())
            .ok()
    }
}
