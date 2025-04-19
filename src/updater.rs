use crate::{color::{GRAY, GREEN, ORANGE, RST}, log_error};
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

async fn get_update() -> Result<Option<Update>, Box<dyn std::error::Error>> {
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

pub fn run_background_check() {
    tokio::spawn(async move {
        match get_update().await {
            Ok(Some(update)) => {
                println!("{GREEN}A new version of serve is available!{RST}");
                println!("➜ {GRAY}version: {RST}v{}", update.version);
                println!("➜ run 'serve --update' to install\n");
            }
            Ok(None) => {}
            Err(e) => log_error!("could not check for updates: {e}"),
        }
    });
}

pub async fn run_update() -> Result<(), String> {
    match get_update().await {
        Ok(Some(upd)) => match install_update(&upd) {
            Ok(_) => println!("\n{GREEN}Update successful!{RST}"),
            Err(e) => return Err(e),
        },
        Ok(None) => println!("{GREEN}v{} - already up to date!{RST}", env!("CARGO_PKG_VERSION")),
        Err(e) => log_error!("could not check for updates: {e}"),
    };
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn install_update(upd: &Update) -> Result<(), String> {
    println!(
        "Updating serve: {ORANGE}v{}{RST} -> {GREEN}v{}{RST}\n",
        env!("CARGO_PKG_VERSION"),
        &upd.version
    );

    #[cfg(feature = "standalone")]
    {
        let current_exe = std::env::current_exe().map_err(|_| "can't access current exe")?;
        github_install(upd, &current_exe)?;
    }

    #[cfg(not(feature = "standalone"))]
    cargo_install()?;

    Ok(())
}

// on windows we need to rename the current binary to replace it
#[cfg(target_os = "windows")]
fn install_update(upd: &Update) -> Result<(), String> {
    use std::process::{Command, Stdio};

    println!(
        "Updating serve: {ORANGE}v{}{RST} -> {GREEN}v{}{RST}\n",
        env!("CARGO_PKG_VERSION"),
        &upd.version
    );

    let current_exe = std::env::current_exe().map_err(|_| "can't access current exe")?;

    let mut backup_path = current_exe.clone();
    backup_path.set_extension("exe.old");

    std::fs::rename(&current_exe, &backup_path)
        .map_err(|e| format!("could not move current exe to backup: {e}"))?;

    #[cfg(feature = "standalone")]
    let res = github_install(upd, &current_exe);

    #[cfg(not(feature = "standalone"))]
    let res = cargo_install();

    if let Err(e) = res {
        std::fs::rename(&backup_path, &current_exe).expect("failed to restore exe");
        return Err(e);
    }

    let parent = backup_path.parent().unwrap();
    let file = backup_path.file_name().unwrap().to_str().unwrap();

    let _ = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &format!("Start-Sleep -Seconds 10; Remove-Item -Path {} -Force", file),
        ])
        .current_dir(parent)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    Ok(())
}

// If serve was installed via cargo, we use cargo to update instead
#[cfg(not(feature = "standalone"))]
pub fn cargo_install() -> Result<(), String> {
    use std::process::{Command, Stdio};

    let crate_name = env!("CARGO_PKG_NAME");
    println!("> cargo install {crate_name}");

    let status = Command::new("cargo")
        .args(["install", crate_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("'cargo install' failed with code: {s}")),
        Err(e) => Err(format!("Failed to run cargo install: {e}")),
    }
}

// In standalone mode the binary is fetched from the github releases
#[cfg(feature = "standalone")]
pub fn github_install(upd: &Update, current_exe: &std::path::Path) -> Result<(), String> {
    use reqwest::blocking::Client;
    use std::{env, fs, io::Write};

    let target_name = if cfg!(windows) { "serve.exe" } else { "serve" };

    let asset = upd
        .release
        .assets
        .iter()
        .find(|a| a.name == target_name)
        .ok_or(format!(
            "'{}' not found in release",
            target_name
        ))?;

    let client = Client::new();

    let res = client
        .get(&asset.browser_download_url)
        .header(
            "User-Agent",
            format!("serve-updater/{}", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .map_err(|e| format!("could not fetch release: {e}"))?;

    println!("Downloading binary...");
    let mut bytes = res
        .bytes()
        .map_err(|e| format!("could not download release: {e}"))?;

    let mut temp_path = current_exe.to_owned();
    temp_path.set_extension("new");

    println!("Creating temporary file...");
    let mut file = fs::File::create(&temp_path)
        .map_err(|e| format!("could not create temp file: {e}"))?;

    println!("Writing to temporary file...");
    file.write_all(&mut bytes)
        .map_err(|e| format!("could not write to file: {e}"))?;

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;

        println!("Setting file executable permissions..");

        let mut perms = file.metadata().unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)
            .map_err(|e| format!("could not set binary permissions: {e}"))?;
    }

    println!("Replacing old binary...");
    fs::rename(&temp_path, &current_exe).map_err(|e| {
        format!("could not replace executable with newer version: {e}")
    })?;

    Ok(())
}
