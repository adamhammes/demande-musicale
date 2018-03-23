extern crate rspotify;

extern crate slack;
use slack::{Event, RtmClient, Message};

extern crate dotenv;
use dotenv::dotenv;
use std::env;

extern crate url;

mod command_handler;
mod command_parser;
mod spotify;

struct MessageHandler {
    channel_name: String,
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
                        if channel != self.channel_name {
                            return;
                        }

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
        println!("on_connect");

        self.channel_name = cli.start_response()
            .channels
            .as_ref()
            .and_then(|channels| {
                channels
                    .iter()
                    .find(|chan| match chan.name {
                        None => false,
                        Some(ref name) => name == "demande-musicale"
                    })
            }).and_then(|chan| chan.id.clone())
            .expect("Couldn't find channel demande-musicale");
        println!("{}", self.channel_name);
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
        handler: command_handler,
        channel_name: "".to_owned(),
    };
    
    let _ = RtmClient::login_and_run(&slack_key, &mut handler);
}
