trait BotCommand {
    /// The name of the command, such as "Add" or "List".
    fn name() -> &str;

    /// The name by which the command is invoked.
    fn command_name() -> &str;

    /// A description of what the command does.
    fn description() -> &str;

    /// Attempts to build the command.
    fn parse(msg: MessageStandard) -> Self;

    /// Handle the command.
    fn handle(&mut self) -> Option<&str>;
}
