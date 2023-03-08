use dotenv::dotenv;
use std::env::var;
pub fn discord_bot_token() -> String {
    dotenv().ok();
    var("TOKEN").expect("discord bot token is not set")
}
