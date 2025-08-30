use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct HelloWorld;

impl HelloWorld {
    pub fn new() -> Self {
        HelloWorld
    }

    pub async fn run(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Hello, world!").await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
}
