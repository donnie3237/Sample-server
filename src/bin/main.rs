use loco_rs::cli;
use lmk::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
