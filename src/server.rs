use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serenity::json::json;
use tide::{Body, Request};
use tokio::sync::RwLock;

use crate::{RecentHistory, api::get_message};

#[derive(Clone)]
struct ServerState {
    history: Arc<RwLock<RecentHistory>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct InferResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

pub fn start_ai_server(history: Arc<RwLock<RecentHistory>>) -> Result<(), tide::Error> {
    let mut app = tide::with_state(ServerState { history });
    app.at("/infer")
        .get(|req: Request<ServerState>| async move {
            let message = get_message(req.state().history.read().await.get())?;
            Ok(json!({
                "message": message
            }))
        });

    // Listen in new thread
    tokio::spawn(async move {
        let mut listener = app.listen("127.0.0.1:8080").await.expect("Start server");
    });

    Ok(())
}
