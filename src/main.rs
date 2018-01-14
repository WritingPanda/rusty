extern crate slack;
use slack::{Event, EventHandler, Message, RtmClient};

struct Handler;

#[allow(unused_variables)]
impl EventHandler for Handler {
  fn on_event(&mut self, cli: &RtmClient, event: Event) {
    println!("on_event(event: {:?})", event);

    match event.clone() {
      Event::Message(message) => self.handle_message(*message, cli, &event),
      _ => return
    };
  }

  fn on_close(&mut self, cli: &RtmClient) {
    println!("on_close");
  }

  fn on_connect(&mut self, cli: &RtmClient) {
    println!("on_connect");
  }
}

#[allow(unused_variables)]
impl Handler {
  fn handle_message(&mut self, message: Message, cli: &RtmClient, event: &Event) {
    let message_standard = match message {
      Message::Standard(message_standard) => message_standard,
      _ => return
    };

    let channel: String = message_standard.channel.unwrap();
    let bot_id: &str = cli.start_response().slf.as_ref().unwrap().id.as_ref().unwrap();
    let text: String = message_standard.text.unwrap();
    if text.contains(bot_id) {
      println!("is a mention");
      respond_hi(&bot_id, &text, &channel, &cli);
    }
  }
}


fn respond_hi(bot_id: &str, text: &str, channel: &str, cli: &RtmClient) {
  let pattern = format!("<@{}> hi", bot_id);

  if text.contains(&pattern) {
    let _ = cli.sender().send_message(channel, "Hi there!");
  }
}

fn main() {
  // get bot token from environment variables
  let env_var = "SLACKBOT_TOKEN";
  let api_key = std::env::vars()
                  .find(|x| x.0 == env_var)
                  .expect(&format!("No {} env var found!", env_var))
                  .1;

  let mut handler = Handler;
  let r = RtmClient::login_and_run(&api_key, &mut handler);
  match r {
    Ok(_) => {}
    Err(err) => panic!("Error: {}", err)
  }
}
