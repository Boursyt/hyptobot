use crate::handler::Handler;
use dotenv::dotenv;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

pub async fn config_api() {
    // Configuration logic for the API with Serenity and le Discord API
    dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {})
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
