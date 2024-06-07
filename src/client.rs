use crate::handler::Handler;
use anyhow::Result;
use serenity::{all::GatewayIntents, Client};

pub async fn build_client(token: &str, intents: GatewayIntents) -> Result<Client> {
    Ok(Client::builder(token, intents)
        .event_handler(Handler)
        .await?)
}
