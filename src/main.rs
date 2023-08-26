use std::env;

use chatgpt::prelude::{ChatGPT, ChatGPTEngine};
use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

struct Handler {
    client: ChatGPT,
}

impl Handler {
    async fn get_response(&self, content: String) -> Result<String, chatgpt::err::Error> {
        let response = self.client.send_message(content).await?;
        return Ok(response.message_choices[0].message.content.to_owned());
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("drewbot") {
            let response = self
                .get_response(msg.content)
                .await
                .unwrap_or("Couldn't get response".to_string());
            msg.channel_id
                .say(&ctx.http, response)
                .await
                .expect("Could not send message");
        }
    }

    async fn ready(&self, _: Context, _: Ready) {
        println!("Ready!");
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected discord token env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Build chatgpt client
    let open_ai_key = env::var("OPENAI_KEY").expect("Could not get openai key");
    let mut client = ChatGPT::new(open_ai_key).expect("Couldn't build chatgpt client");
    client.config.engine = ChatGPTEngine::Gpt35Turbo;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { client })
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e.to_string());
    }
}
