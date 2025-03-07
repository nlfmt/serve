use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::models::DirectoryContent;

pub fn pretty_path(path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        let mut path_str = path.to_str().unwrap().to_owned();
        if let Some(p) = path_str.strip_prefix(r"\\?\") {
            path_str = p.to_string()
        }
        path_str
    }
    #[cfg(not(target_os = "windows"))]
    {
        path.to_str().unwrap().to_owned()
    }
}

pub fn read_entries(path: &Path) -> std::io::Result<DirectoryContent> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            let meta = entry.metadata().ok()?;
            Some((file_name, meta.is_dir()))
        })
        .fold(DirectoryContent::default(), |mut res, (name, is_dir)| {
            if is_dir {
                res.dirs.push(name)
            } else {
                res.files.push(name);
            }
            res
        }))
}

pub fn parse_relative_path(root: &Path, path: &str) -> Result<PathBuf, String> {
    if !path.starts_with("./") {
        return Err("Path is not relative".to_string());
    }

    let path = root
        .join(path)
        .canonicalize()
        .map_err(|_| String::from("Invalid Path"))?;

    match path.starts_with(root) {
        true => Ok(path),
        false => Err(String::from("Path is not relative to the root folder")),
    }
}
