use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref CRINGE_REGEX: Regex = Regex::new(r"(?im)\bcringe\b").unwrap();
}
const CRINGE_RESPONSE: &str = "#42977 It's so funny to see manchildren use the word cringe everywhere. They're like 10 year olds calling anything they don't like \"gross\". But cringer.";

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if CRINGE_REGEX.is_match(&msg.content) && !msg.author.bot {
            if let Err(why) = msg.channel_id.say(&ctx.http, CRINGE_RESPONSE) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
