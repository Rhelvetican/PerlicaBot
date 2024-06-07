mod client;
mod cmd;
mod config;
mod handler;
mod intents;
mod utils;

use anyhow::Result;
use client::build_client;
use config::get_token;
use intents::get_intents;

#[tokio::main]
async fn main() -> Result<()> {
    let token = get_token()?;
    let mut client = build_client(&token, get_intents()).await?;
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}
