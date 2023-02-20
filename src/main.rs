use std::sync::Arc;

use once_cell::sync::Lazy;
use reqwest::Client;
use serde_json::{json, Value};
use teloxide::prelude::*;

static API_URL: Lazy<String> = Lazy::new(|| std::env::var("API_URL").unwrap());

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let client = Arc::new(Client::new());
    let bot = Bot::from_env();
    let client = client.clone();

    teloxide::repl(bot, move |bot: Bot, message: Message| {
        chat_handler(client.clone(), bot, message)
    })
    .await;
}

async fn chat_handler(client: Arc<Client>, bot: Bot, message: Message) -> ResponseResult<()> {
    let chat_id = message.chat.id;
    if let Some(text) = message.text() {
        if text.starts_with('/') {
            return Ok(());
        }

        let response = client
            .post(format!("{}/message/tg-{}-default", *API_URL, chat_id))
            .json(&json!({ "message": text }))
            .send()
            .await?;
        if response.status() != 200 {
            bot.send_message(chat_id, "[SERIKA ERROR] API error")
                .reply_to_message_id(message.id)
                .await?;
            return Ok(());
        }

        // on success
        let data = response.json::<Value>().await.unwrap();
        let response_text = data["response"].as_str().unwrap();
        bot.send_message(chat_id, response_text).await?;
    } else {
        bot.send_message(chat_id, "[SERIKA ERROR] Empty chat message")
            .reply_to_message_id(message.id)
            .await?;
    }

    Ok(())
}
