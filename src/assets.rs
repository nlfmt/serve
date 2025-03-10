use rust_embed::Embed;

#[derive(Embed)]
#[folder = "./app/dist"]
pub struct Assets;
