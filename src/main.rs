mod config;



use serenity::async_trait;
use serenity::framework::standard::macros::{ command, group,  hook};
use serenity::framework::standard::{
    CommandResult, StandardFramework,
};
use serenity::model::channel::{ Message};
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::prelude::*;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "{} is successfully connected to discord gateway!",
            ready.user.name
        );
    }
}

#[group]
#[commands(ping, about)]
struct General;

#[command]
// Limit command usage to guilds.
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong! : )").await?;

    Ok(())
}

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "This is a small test-bot! : )")
        .await?;

    Ok(())
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

#[tokio::main]
async fn main() {
    let token = config::discord_bot_token();

    let intents = GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES
    | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .prefix("~")
                .delimiters(vec![", ", ","])
        })
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
