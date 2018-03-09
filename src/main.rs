extern crate slack;
use slack::{Event, RtmClient, Message};

extern crate dotenv;
use dotenv::dotenv;
use std::env;

struct MessageHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MessageHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        match event {
            Event::Message(msg_event) => {
                if let Message::Standard(msg) = *msg_event {
                    println!("{:?}", msg.text);
                    println!("{:?}", msg.user);
                }
            },
            _ => {}
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
    }

    fn on_connect(&mut self, cli: &RtmClient) {
    }
}

fn main() {
    dotenv().ok();
    let api_key = env::var("BOT_ACCESS_TOKEN")
        .expect("Couldn't read environment variable BOT_ACCESS_TOKEN");    

    let mut handler = MessageHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);

    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
