use std::env;
use std::time::Duration;

use futures::StreamExt;
use log::error;
use telegram_bot::*;

async fn handle_message(api: &Api, message: Message) {
    match message.kind {
        MessageKind::NewChatMembers { .. } | MessageKind::LeftChatMember { .. } => {
            tokio::spawn(api.send(message.delete()));
        }
        _ => (),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let bot_key = env::var("TELEGRAM_BOT_KEY").expect("specify bot key in TELEGRAM_BOT_KEY");
    let api = Api::new(bot_key);

    let mut stream = api.stream();
    stream
        .error_delay(Duration::from_secs(30))
        .timeout(Duration::from_secs(35));
    while let Some(update) = stream.next().await {
        match update {
            Ok(Update {
                kind: UpdateKind::Message(message),
                ..
            }) => handle_message(&api, message).await,
            Err(error) => error!("{:?}", error),
            _ => (),
        }
    }
}
