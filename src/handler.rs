use crate::commandes::pong;
use crate::commandes::*;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Handler {}
impl Handler {
    pub async fn new() -> Self {
        // Initialize Handler here if needed
        Handler {}
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let hello_world = hello_world::HelloWorld::new();
        let pong = pong::Pong::new();
        let paff = paff::Paff::new();

        match msg.content.as_str() {
            "ping" => {
                pong.ping(ctx, msg).await;
            }
            "hello" | "Hello" => {
                hello_world.run(ctx, msg).await;
            }
            "paff" => {
                paff.run(ctx, msg).await;
            }

            _ => {}
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
