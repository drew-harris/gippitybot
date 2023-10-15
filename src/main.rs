mod api;
mod server;

use std::{env, sync::Arc};

use rand::Rng;
use serenity::{
    async_trait,
    model::prelude::{Message, Ready},
    Client,
};

use serenity::prelude::*;
use server::start_ai_server;

struct Handler {
    history: Arc<RwLock<RecentHistory>>,
}

fn get_random_int(max: u32) -> u32 {
    rand::thread_rng().gen_range(0..max)
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // If message is not from bot save it
        if msg.author.bot {
            return;
        }

        // Add message, scoped to quickly unlock rwlock
        {
            self.history.write().await.add(msg.content.clone());
        }

        let msg_amount = self.history.read().await.messages.len();
        if (get_random_int(5) != 5 || msg_amount < 5) && msg.content != "!test" {
            return;
        }

        let message = api::get_message(self.history.read().await.get());

        match message {
            Ok(res_msg) => msg.reply(ctx.http, res_msg).await,
            Err(_) => msg.reply(ctx.http, "error").await,
        }
        .expect("sends message");
    }

    async fn ready(&self, _c: Context, r: Ready) {
        println!("Logged in as {}", r.user.tag());
    }
}

pub struct RecentHistory {
    messages: Vec<String>,
    limit: usize,
}

impl RecentHistory {
    fn new(keep: usize) -> Self {
        Self {
            messages: Vec::new(),
            limit: keep,
        }
    }

    fn add(&mut self, message: String) {
        self.messages.push(message);
        if self.messages.len() > self.limit {
            self.messages.remove(0);
        }
    }

    fn get(&self) -> Vec<String> {
        self.messages.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected discord token env");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let history = RwLock::new(RecentHistory::new(20));
    let harc = Arc::new(history);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            history: Arc::clone(&harc),
        })
        .await
        .expect("Could not create client");

    start_ai_server(harc).expect("START SERVER");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e.to_string());
    }
}
