use qrcode_generator::QrCodeEcc;
use rocket::{http::ContentType, State};

use crate::{auth::AuthGuard, state::AppState, utils::connection_string};

#[get("/qr")]
pub fn get_connection_qrcode(_auth: AuthGuard, state: &State<AppState>) -> (ContentType, Vec<u8>) {
    (
        ContentType::PNG,
        qrcode_generator::to_png_to_vec(
            connection_string(state.interface, state.port),
            QrCodeEcc::Low,
            1024,
        )
        .unwrap(),
    )
}
