use std::net::IpAddr;
use rocket::{config::Ident, data::{Limits, ToByteUnit}, Config};
use rocket_cors::AllowedOrigins;
use crate::{auth::AuthFairing, routes, state::AppState, ServeArgs};

pub async fn launch_server(args: ServeArgs, app_state: AppState) -> anyhow::Result<()> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        ..Default::default()
    }
    .to_cors()?;
    let cfg = get_config(args.interface, args.port);

    let server = rocket::custom(cfg)
        .manage(app_state)
        .attach(AuthFairing)
        .attach(cors)
        .mount(
            "/api",
            routes![
                routes::get_dir::get_dir_content,
                routes::download::download_file,
                routes::download_folder::download_folder,
                routes::upload::pre_upload_file,
                routes::upload::upload_file,
                routes::settings::get_settings,
                routes::get_qrcode::get_connection_qrcode,
                routes::file_ops::rename,
                routes::file_ops::delete,
                routes::file_ops::move_item,
                routes::get_entry_properties::get_entry_properties,
                routes::create_folder::create_folder,
            ],
        )
        .mount("/", routes![routes::get_embedded_file::get_embedded_file])
        .launch();

    server.await?;
    Ok(())
}


fn get_config(address: IpAddr, port: u16) -> Config {
    Config {
        address,
        port,
        ident: Ident::try_new("serve").unwrap(),

        limits: Limits::default()
            .limit("data-form", 10.gigabytes())
            .limit("file", 10.gigabytes()),

        log_level: if cfg!(debug_assertions) {
            rocket::config::LogLevel::Normal
        } else {
            rocket::config::LogLevel::Off
        },
        ..Default::default()
    }
}