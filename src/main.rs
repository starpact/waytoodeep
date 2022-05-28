mod dumb;

use color_eyre::Report;
use reqwest::Client;
use tracing::info;
use tracing_subscriber::EnvFilter;

const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    let client = Client::new();
    tokio::try_join!(fetch_thing(&client, URL_1), fetch_thing(&client, URL_2))?;

    Ok(())
}

async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
    let resp = client.get(url).send().await?.error_for_status()?;
    info!(%url, content_type = ?resp.headers().get("content-type"), "Got a respone!");

    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
