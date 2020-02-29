mod library;
mod reader;

#[macro_use]
extern crate prettytable;
use env_logger;
use library::{last_five, parse_put};
use log::info;
use reader::read_feeds;
use slack::{Event, EventHandler, Message, RtmClient};

struct Handler;

#[derive(Clone, Debug)]
pub enum SlackChannel {
    Aws,
    Rust,
    Kubernetes,
    Python,
    BattleBots,
}

impl SlackChannel {
    pub fn id(&self) -> &'static str {
        match self {
            SlackChannel::Aws => "CA6MUA4LU",
            SlackChannel::Rust => "C8EHWNKHV",
            SlackChannel::Kubernetes => "C91DM9Y6S",
            SlackChannel::Python => "C6DTBQK4P",
            SlackChannel::BattleBots => "CD31RPEFR",
        }
    }
}

fn bot_say(channel: SlackChannel, msg: &str) {
    let api_client = slack_api::requests::default_client().unwrap();
    let token: String = std::env::vars()
        .filter(|(k, _)| k == "SLACKBOT_TOKEN")
        .map(|(_, v)| v)
        .collect();
    let chan_id = channel.id();
    let bot_msg = format!("```{}```", msg);

    let mut msg = slack_api::chat::PostMessageRequest::default();
    msg.channel = &chan_id;
    msg.text = &bot_msg;
    msg.as_user = Some(true);

    info!(
        "{:?}",
        slack_api::chat::post_message(&api_client, &token, &msg)
    );
}

#[allow(unused_variables)]
impl EventHandler for Handler {
    fn on_event(&mut self, client: &RtmClient, event: Event) {
        info!("on_event(event: {:?})", event);

        if let Event::Message(message) = event.clone() {
            self.handle_message(*message, client, &event)
        }
    }

    fn on_close(&mut self, client: &RtmClient) {}

    fn on_connect(&mut self, client: &RtmClient) {
        // std::thread::spawn(read_feeds);
    }
}

#[allow(unused_variables)]
impl Handler {
    fn handle_message(&mut self, message: Message, client: &RtmClient, event: &Event) {
        let message_standard = match message {
            Message::Standard(message_standard) => message_standard,
            _ => return,
        };

        let channel: String = message_standard.channel.unwrap();
        let user: String = message_standard.user.unwrap();
        let bot_id: &str = client
            .start_response()
            .slf
            .as_ref()
            .unwrap()
            .id
            .as_ref()
            .unwrap();

        let text: String = message_standard.text.unwrap();
        if text.starts_with("!put ") {
            info!("matched !put");
            parse_put(&text, &user)
        } else if text.starts_with("!last") {
            info!("matched !last");
            last_five()
        } else if text.contains(bot_id) {
            info!("is a mention");
            respond_hi(&bot_id, &text, &channel, &client);
        }
    }
}

fn respond_hi(bot_id: &str, text: &str, channel: &str, client: &RtmClient) {
    let pattern = format!("<@{}> hi", bot_id);

    if text.contains(&pattern) {
        let _ = client.sender().send_message(channel, "Hi there!");
    }
}

fn main() {
    env_logger::init();
    // https://github.com/emk/rust-musl-builder#making-openssl-work
    openssl_probe::init_ssl_cert_env_vars();

    // get bot token from environment variables
    let target_env_var = "SLACKBOT_TOKEN";
    let mut api_key: String = "".to_string();
    for (k, v) in std::env::vars() {
        if k == target_env_var {
            api_key = v;
        }
    }

    if api_key.is_empty() {
        eprintln!(
            "no {} environment variable found!\nPlease set this env var and try again.",
            target_env_var
        );
        std::process::exit(1);
    }

    let mut handler = Handler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
