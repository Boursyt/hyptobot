use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Pong;

impl Pong {
    pub fn new() -> Self {
        Pong
    }

    pub async fn ping(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
}
