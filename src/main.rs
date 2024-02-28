use log::info;
use teloxide::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use tokio::sync::Mutex as TokioMutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Starting bot...");

    let bot = Bot::from_env();

    let last_person_id: Arc<TokioMutex<Option<String>>> = Arc::new(TokioMutex::new(None));

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let last_person_id = Arc::clone(&last_person_id);
        async move {
            // Check if the message is a sticker
            if let Some(_sticker) = msg.sticker() {
                // Check if the sticker is sent from the same person
                let mut last_person_id = last_person_id.lock().await;
                if let Some(last_id) = last_person_id.as_ref() {
                    if last_id == &msg.from().unwrap().id.to_string() {
                        info!("Sticker sent from same person! Deleting...");
                        bot.delete_message(msg.chat.id, msg.id).await?;
                        return Ok(());
                    }
                }
                info!("New sticker sent, saving user ID...");
                *last_person_id = Some(msg.from().unwrap().id.to_string());
            }
            Ok(())
        }
    })
    .await;
}