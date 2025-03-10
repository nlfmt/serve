use std::path::PathBuf;

use actix_multipart::form::{bytes::Bytes, tempfile::TempFile, text::Text, MultipartForm};
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub file_dir: PathBuf,
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

#[derive(Deserialize)]
pub struct FilesQuery {
    pub path: String,
}

#[derive(Deserialize)]
pub struct DownloadQuery {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct UploadQuery {
    pub path: String,
    pub file_name: String,
    pub overwrite: bool,
}