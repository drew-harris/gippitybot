use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
    Client as ChatGPT,
};
use std::env;

use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    Client,
};

use serenity::prelude::*;

struct Handler {
    ai_client: ChatGPT<OpenAIConfig>,
}

async fn get_ai_response(client: &ChatGPT<OpenAIConfig>, msg: &str) -> Result<String, OpenAIError> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("llama3-70b-8192")
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(msg)
            .build()?
            .into()])
        .max_tokens(120_u16)
        .build()?;

    let response = client
        .chat() // Get the API "group" (completions, images, etc.) from the client
        .create(request)
        .await?; // Make the API call in that "group"

    // TODO: WTF
    Ok(response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone()
        .unwrap())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if !msg.mentions_me(&ctx).await.unwrap() {
            return;
        }

        let msg_content = msg.content.clone();

        // Get my current id
        let my_id = ctx.http.get_current_user().await.unwrap().id;
        // Remove the mention from the message
        let msg_content = msg_content.replace(&format!("<@{}>", my_id), "");

        let response_text = get_ai_response(&self.ai_client, &msg_content).await;

        match response_text {
            Ok(response) => {
                msg.reply(&ctx.http, response)
                    .await
                    .expect("Could not send message");
            }

            Err(err) => {
                // Print error and also send it to the user
                let err_msg = format!("Error: {}", err);
                msg.reply(&ctx.http, err_msg)
                    .await
                    .expect("Could not send message");
                println!("Error: {}", err);
            }
        };
    }

    async fn ready(&self, _c: Context, r: Ready) {
        println!("Logged in as {}", r.user.tag());
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected discord token env");
    let groq_token = env::var("GROQ_TOKEN").expect("Expected groq token env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let ai_config = OpenAIConfig::new()
        .with_api_base("https://api.groq.com/openai/v1")
        .with_api_key(groq_token);

    let ai_client = ChatGPT::with_config(ai_config);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { ai_client })
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e.to_string());
    }
}
