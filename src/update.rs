use semver::Version;
use serde::Deserialize;

pub const REPO: &str = "nlfmt/serve";

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    #[cfg(feature = "standalone")]
    pub assets: Vec<Asset>,
}

#[cfg(feature = "standalone")]
#[derive(Deserialize)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
}

pub struct Update {
    pub version: Version,
    #[cfg(feature = "standalone")]
    pub release: Release,
}

pub async fn check_for_update() -> Result<Option<Update>, Box<dyn std::error::Error>> {
    use reqwest::Client;

    let client = Client::new();
    let url = format!("https://api.github.com/repos/{REPO}/releases/latest");

    let release = client
        .get(&url)
        .header(
            "User-Agent",
            format!("serve-updater/{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await?
        .json::<Release>()
        .await?;

    let latest = Version::parse(release.tag_name.trim_start_matches('v'))?;
    let current = Version::parse(env!("CARGO_PKG_VERSION"))?;

    if latest > current {
        Ok(Some(Update {
            #[cfg(feature = "standalone")]
            release,
            version: latest,
        }))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "standalone")]
pub fn update(upd: Update) {
    use std::{env, fs, io::Write};

    use crate::color::{GREEN, ORANGE, RST};
    use reqwest::blocking::Client;

    println!("Updating serve to {GREEN}v{}{RST}\n", &upd.version);

    let target_name = if cfg!(windows) { "serve.exe" } else { "serve" };

    let asset = match upd
        .release
        .assets
        .into_iter()
        .find(|a| a.name == target_name)
    {
        Some(a) => a,
        None => {
            eprintln!("{ORANGE}Error: '{}' not found in release{RST}", target_name);
            return;
        }
    };

    let client = Client::new();

    let res = match client
        .get(asset.browser_download_url)
        .header(
            "User-Agent",
            format!("serve-updater/{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
    {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{ORANGE}Error: could not fetch release: {e}{RST}");
            return;
        }
    };

    println!("Downloading binary...");
    let mut bytes = match res.bytes() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{ORANGE}Error: could not download release: {e}{RST}");
            return;
        }
    };

    let current_exe = match env::current_exe() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{ORANGE}Error: could not access current executable: {e}{RST}");
            return;
        }
    };
    
    let mut temp_path = current_exe.clone();
    temp_path.set_extension("new");
    
    println!("Creating temporary file...");
    let mut file = match fs::File::create(&temp_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{ORANGE}Error: could not create temp : {e}{RST}");
            return;
        }
    };

    println!("Writing to temporary file...");
    if let Err(e) = file.write_all(&mut bytes) {
        eprintln!("{ORANGE}Error: could not write to file: {e}{RST}");
        return;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        println!("Setting file executable permissions..");

        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        if let Err(e) = fs::set_permissions(&temp_path, perms) {
            eprintln!("{ORANGE}Error: could not set binary permissions: {e}{RST}");
            return;
        }
    }

    println!("Replacing old binary...");
    if let Err(e) = fs::rename(&temp_path, &current_exe) {
        eprintln!("{ORANGE}Error: could not replace executable with newer version: {e}{RST}");
        return;
    }

    println!("\n{GREEN}Successfully updated serve via github!{RST}");
}

#[cfg(not(feature = "standalone"))]
pub fn update(upd: Update) {
    use crate::color::{GREEN, ORANGE, RST};
    use std::process::{Command, Stdio};

    let crate_name = env!("CARGO_PKG_NAME");

    println!("Updating serve to {GREEN}v{}{RST}\n", &upd.version);
    println!("> cargo install {crate_name}");

    let status = Command::new("cargo")
        .args(["install", crate_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("\n{GREEN}Successfully updated serve via cargo!{RST}");
        }
        Ok(s) => eprintln!("\n{ORANGE}Error: cargo install failed with exit code: {s}{RST}"),
        Err(e) => eprintln!("\n{ORANGE}Error: Failed to run cargo install: {e}{RST}"),
    }
}
