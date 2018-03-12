extern crate regex;
use self::regex::Regex;

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

        let re_match = regex.captures(input);

        re_match.map(|caps| AddCommand {
            uri: caps["uri"].to_owned(),
            msg: "temp message".to_owned(),
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