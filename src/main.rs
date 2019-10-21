use std::env;

use env_logger;
use futures::stream::Stream;
use futures::Future;
use log::error;

use telebot::bot;
use telebot::functions::*;
use telebot::objects as tbo;

fn handle_update((tgbot, update): (bot::RequestHandle, tbo::Update)) -> Result<(), ()> {
    if let tbo::Update {
        message: Some(tbo::Message {
            message_id,
            chat: tbo::Chat { id: chat_id, .. },
            new_chat_member: opt_new,
            left_chat_member: opt_left,
            ..
        }),
        ..
    } = update {
        if opt_new.is_some() || opt_left.is_some() {
            tokio::spawn(tgbot.delete_message(chat_id, message_id).send()
                .map(|_| ())
                .map_err(|e| error!("{}", e)));
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();
    let bot = bot::Bot::new(&env::var("TELEGRAM_BOT_KEY").unwrap()).update_interval(200);

    tokio::run(bot.resolve_name()
        .map_err(|e| error!("{}", e))
        .and_then(|name| bot
            .get_stream(name)
            .map_err(|e| error!("{}", e))
            .for_each(handle_update)));
}
