extern crate rspotify;

extern crate slack;
use slack::{Event, RtmClient, Message};

extern crate dotenv;
use dotenv::dotenv;
use std::env;

mod command_handler;
mod command_parser;
mod spotify;

struct MessageHandler {
    handler: command_handler::Commander,
}

#[allow(unused_variables)]
impl slack::EventHandler for MessageHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("{:?}", event);
        match event {
            Event::Message(msg_event) => {
                if let Message::Standard(msg) = *msg_event {
                    if let (Some(txt), Some(user), Some(channel)) = (msg.text, msg.user, msg.channel) {
                        let command = command_parser::parse_command(&txt, &user);
                        let response = self.handler.handle(command);

                        if let Some(msg) = response {
                            let _ = cli.sender().send_message(&channel, &msg);
                        }
                    }
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
    let slack_key = env::var("SLACK_BOT_ACCESS_TOKEN")
        .expect("Couldn't read environment variable SLACK_BOT_ACCESS_TOKEN");    
    
    let spotify = spotify::SpotifyWrapper::new();
    let command_handler = command_handler::Commander {
        spotify: spotify
    };
    
    let mut handler = MessageHandler {
        handler: command_handler
    };
    
    let _ = RtmClient::login_and_run(&slack_key, &mut handler);
}
