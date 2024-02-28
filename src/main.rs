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

    let last_sticker_id: Arc<TokioMutex<Option<String>>> = Arc::new(TokioMutex::new(None));

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let last_sticker_id = Arc::clone(&last_sticker_id);
        async move {
            // Check if the message is a sticker
            if let Some(sticker) = msg.sticker() {
                // Check if the sticker is the same as the last one
                let mut last_sticker_id = last_sticker_id.lock().await;
                if let Some(last_id) = last_sticker_id.as_ref() {
                    if last_id == &sticker.file.unique_id {
                        info!("Same sticker sent again! Deleting...");
                        bot.delete_message(msg.chat.id, msg.id).await?;
                        return Ok(());
                    }
                }
                info!("New sticker sent, saving it...");
                *last_sticker_id = Some(sticker.file.unique_id.clone());
            }
            Ok(())
        }
    })
    .await;
}