use clap::Parser;
use nlfmt_serve::ServeArgs;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let args = ServeArgs::parse();

    nlfmt_serve::run(args).await
}
