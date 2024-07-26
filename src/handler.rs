use std::sync::LazyLock;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::CONFIG;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let cfg = LazyLock::force(&CONFIG);
        if msg.content.starts_with(&cfg.cmd_prefix) {
            let cmd = msg.content.strip_prefix(&cfg.cmd_prefix).unwrap_or("");
            match cmd {
                "pong" => {
                    reply(&ctx, &msg, "Ping!").await;
                }
                "ping" => {
                    reply(&ctx, &msg, "Pong!").await;
                }
                _ => {}
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn reply(ctx: &Context, msg: &Message, content: &str) {
    if let Err(why) = msg.channel_id.say(ctx.http.clone(), content).await {
        println!("Error sending message: {:?}", why);
    }
}
