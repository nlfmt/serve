use std::{
    env::current_dir,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn generate_temp_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let filename = format!("serve_temp_{}", timestamp);
    std::env::temp_dir().join(filename)
}

fn path_contains_symlink(path: &Path) -> io::Result<bool> {
    let mut current = PathBuf::new();

    for component in path.components() {
        current.push(component);

        // Check if this component is a symlink
        if let Ok(metadata) = fs::symlink_metadata(&current) {
            if metadata.file_type().is_symlink() {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

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

pub fn get_root_dir(path: &Option<String>) -> Result<PathBuf, std::io::Error> {
    let root_dir = path
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            current_dir()
                .expect("Can't access current directory")
                .to_path_buf()
        })
        .canonicalize()?;

    Ok(root_dir)
}

pub fn parse_relative_path(root: &Path, path: &str, allow_symlinks: bool) -> Option<PathBuf> {
    use std::path::Component;

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

    match allow_symlinks {
        true => Some(joined),
        false => {
            if path_contains_symlink(&joined).ok()? {
                None
            } else {
                Some(joined)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Path::new("C:/test");
        assert!(parse_relative_path(&root, "/abc/def", false).is_some());
        assert!(parse_relative_path(&root, "./abc/def", false).is_some());
        assert!(parse_relative_path(&root, r"\abc\def", false).is_some());
        assert!(parse_relative_path(&root, r".\abc\def", false).is_some());
        assert!(parse_relative_path(&root, "/abc/../def", false).is_some());
        assert!(parse_relative_path(&root, "abc/../def", false).is_some());
        assert!(parse_relative_path(&root, "abc/.././def/../def/../", false).is_some());

        // prefixes should not be allowed
        assert!(parse_relative_path(&root, "C:/abc/def", false).is_none());
        assert!(parse_relative_path(&root, "D:/abc/def", false).is_none());

        // paths that move beyond the root dir at any point should not be allowed
        assert!(parse_relative_path(&root, "abc/../../def", false).is_none());
        assert!(parse_relative_path(&root, "abc/.././def/../def/../../", false).is_none());
    }
}