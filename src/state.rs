use std::{net::IpAddr, path::{Path, PathBuf}};

use crate::{auth::Auth, ServeArgs};


pub struct AppState {
    pub root_dir: PathBuf,
    pub port: u16,
    pub interface: IpAddr,

    pub upload: bool,
    pub symlinks: bool,

    pub overwrite: bool,
    pub rename: bool,
    pub delete: bool,

    pub auths: Vec<Auth>,
}

impl AppState {
    pub fn new(args: &ServeArgs, root_dir: &Path) -> Self {
        let auths: Vec<Auth> = args
            .auth
            .iter()
            .chain(args.auth_file.iter().flatten())
            .cloned()
            .collect();

        AppState {
            root_dir: root_dir.to_path_buf(),
            port: args.port,
            interface: args.interface,
            symlinks: args.symlinks,
            upload: args.upload,
            overwrite: args.overwrite || args.modify,
            delete: args.delete || args.modify,
            rename: args.rename || args.modify,
            auths,
        }
    }

    pub fn get_perms(&self) -> Vec<String> {
        let mut perms = Vec::new();
        if self.symlinks {
            perms.push("symlinks".to_string())
        }
        if self.upload {
            perms.push("upload".to_string())
        }
        if self.overwrite {
            perms.push("overwrite".to_string())
        }
        if self.rename {
            perms.push("rename".to_string())
        }
        if self.delete {
            perms.push("delete".to_string())
        }
        perms
    }
}