use std::{net::IpAddr, path::PathBuf};

use rocket::{fs::NamedFile, http::Header, response, serde::Serialize, Response};

use crate::auth::Auth;

pub struct AppState {
    pub root_dir: PathBuf,
    pub port: u16,
    pub interface: IpAddr,

    pub upload: bool,
    pub symlinks: bool,

    pub overwrite: bool,
    pub rename: bool,
    pub delete: bool,
}

pub struct AuthState {
    pub auths: Vec<Auth>
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

#[derive(FromForm)]
pub struct UploadQuery {
    pub path: String,
    pub file_name: String,
    pub overwrite: bool,
}

pub struct FileResponse {
    pub inner: NamedFile,
    pub file_name: String,
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

#[derive(Serialize)]
pub struct Settings {
    pub upload: bool,
    pub overwrite: bool,
    pub rename: bool,
    pub delete: bool,
}

#[derive(Serialize)]
pub struct EntryProperties {
    pub accessed: Option<u64>,
    pub readonly: bool,
}