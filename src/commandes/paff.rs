use serenity::model::prelude::*;
use serenity::prelude::*;
pub struct Paff;

impl Paff {
    pub fn new() -> Self {
        Paff
    }

    pub async fn run(&self, ctx: Context, msg: Message) {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Maxi paff deluxe").await {
            eprintln!("Error sending message: {:?}", why);
        }
    }
}
