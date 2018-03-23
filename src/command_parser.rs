extern crate regex;
use self::regex::Regex;

use url::{Host,Url};

#[derive(Debug)]
pub enum Command {
    Add(AddCommand),
    List,
    Error,
}

#[derive(Debug)]
pub struct AddCommand {
    pub uri: String,
    msg: String,
    user: String,
}

impl AddCommand {
    fn parse(input: &str, user: &str) -> Option<AddCommand> {
        let regex = Regex::new(r"!add\s+(?P<uri>\S*)").unwrap();

        let caps = regex.captures(input)?;

        let patterns: &[_] = &['<', '>'];
        let trimmed = caps["uri"].trim_matches(patterns);

        let uri = parse_uri(trimmed)?;


        Some(AddCommand {
            uri: uri.to_owned(),
            msg: "tmp message".to_owned(),
            user: user.to_owned(),
        })
    }
}

#[derive(Debug)]
pub struct ListCommand;

impl ListCommand {
    pub fn parse(input: &str) -> Option<ListCommand> {
        if input.starts_with("!list") {
            Some(ListCommand)
        } else {
            None
        }
    }
}

pub fn parse_command(input: &str, user: &str) -> Command {
    if let Some(command) = AddCommand::parse(input, user) {
        println!("add");
        Command::Add(command)
    } else if let Some(_) = ListCommand::parse(input) {
        println!("list");
        Command::List
    } else {
        println!("error");
        Command::Error
    }
}

fn parse_uri(input: &str) -> Option<String> {
    let uri = Url::parse(input).ok()?;

    match (uri.scheme(), uri.host()) {
        ("spotify", _) => {
            Some(input.to_owned())
        }
        ("https", Some(Host::Domain("open.spotify.com"))) => {
            let cloned = uri.clone();
            let path = cloned.path().to_owned();
            Some(path)
        }
        (_, _) => {
            None
        }
    }
}
