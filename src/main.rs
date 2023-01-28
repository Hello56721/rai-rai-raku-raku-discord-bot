use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude as discord;

use std::process::Command;
use tokio::sync::Mutex;

#[derive(Default)]
struct Bot {
    id: UserId
}

struct EventHandler {
    bot: Mutex<Bot>
}

#[serenity::async_trait]
impl discord::EventHandler for EventHandler {
    async fn ready(&self, _: discord::Context, ready: Ready) {
        println!("[INFO]: The bot has logged on as {}", ready.user.name);
        
        let mut bot = self.bot.lock().await;
        bot.id = ready.user.id;
    }

    async fn message(&self, context: discord::Context, message: Message) {
        println!("[MESSAGE]: {}", message.content);
        
        let bot = self.bot.lock().await;
        
        if bot.id == message.author.id {
            return;
        }
        
        if message.content.starts_with("I am") {
            if let Err(why) = message.reply_ping(context, "fuk yo").await {
                println!(
                    "[ERROR]: Failed to reply to a message. Here's why:\n{:?}",
                    why
                );
            }
        } else if message.content == "$$$restart$$$" && message.author.id == 650439182204010496 {
            Command::new("cargo").arg("run").spawn().unwrap();
            
            std::process::exit(0);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("user/token.txt").expect("Failed to load the token file.");

    let intents = discord::GatewayIntents::GUILDS
        | discord::GatewayIntents::GUILD_MESSAGES
        | discord::GatewayIntents::MESSAGE_CONTENT;
        
    let event_handler = EventHandler { bot: Mutex::new(Bot::default()) };

    let mut bot = discord::Client::builder(&token, intents)
        .event_handler(event_handler)
        .await
        .expect("Failed to create the client. Perhaps the token wasn't valid?");

    bot.start().await.expect("Failed to start the bot.");
}
