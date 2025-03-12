use std::{
    fmt::Display,
    fs,
    os::windows::fs::MetadataExt,
    path::{Component, Path, PathBuf},
    time::UNIX_EPOCH,
};

use crate::models::{DirectoryContent, FileInfo, FolderInfo};

pub fn pretty_path(path: &Path) -> impl Display {
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
        path.display()
    }
}

pub fn read_entries(path: &Path) -> std::io::Result<DirectoryContent> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            let meta = entry.metadata().ok()?;
            Some((file_name, meta))
        })
        .fold(DirectoryContent::default(), |mut res, (name, meta)| {
            let size = meta.file_size();
            let modified = meta
                .modified()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));
            let created = meta
                .created()
                .ok()
                .and_then(|v| Some(v.duration_since(UNIX_EPOCH).ok()?.as_secs()));
            let is_symlink = meta.is_symlink();

            if meta.is_dir() {
                res.dirs.push(FolderInfo {
                    name,
                    modified,
                    created,
                    is_symlink,
                })
            } else {
                res.files.push(FileInfo {
                    name,
                    size,
                    modified,
                    created,
                    is_symlink,
                });
            }
            res
        }))
}

pub fn parse_relative_path(root: &Path, path: &str) -> Option<PathBuf> {
    let path = Path::new(path);
    let components = path.components();
    let mut stack = Vec::new();

    for c in components {
        match c {
            Component::Prefix(_) => {
                return None;
            }
            Component::CurDir | Component::RootDir => {}
            Component::ParentDir => {
                if stack.pop().is_none() {
                    return None;
                }
            }
            Component::Normal(part) => {
                stack.push(part.to_str()?);
            }
        }
    }

    let joined = root.join(stack.join("/"));

    Some(joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Path::new("C:/test");
        assert!(parse_relative_path(&root, "/abc/def").is_some());
        assert!(parse_relative_path(&root, "./abc/def").is_some());
        assert!(parse_relative_path(&root, r"\abc\def").is_some());
        assert!(parse_relative_path(&root, r".\abc\def").is_some());
        assert!(parse_relative_path(&root, "/abc/../def").is_some());
        assert!(parse_relative_path(&root, "abc/../def").is_some());
        assert!(parse_relative_path(&root, "abc/.././def/../def/../").is_some());

        // prefixes should not be allowed
        assert!(parse_relative_path(&root, "C:/abc/def").is_none());
        assert!(parse_relative_path(&root, "D:/abc/def").is_none());

        // paths that move beyond the root dir at any point should not be allowed
        assert!(parse_relative_path(&root, "abc/../../def").is_none());
        assert!(parse_relative_path(&root, "abc/.././def/../def/../../").is_none());
    }
}
