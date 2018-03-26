use command::BotCommand;

pub struct AddCommand {
    pub uri: String,
    msg: String,
    user: String,
}

impl BotCommand for AddCommand {
    fn name() -> &str {
        "Add Song"
    }

    fn command_name() -> &str {
        "add"
    }

    fn description() -> &str {
        "Add a song to the queue"
    }

    fn parse() -> &str {

    }
}