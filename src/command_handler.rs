use command_parser::{AddCommand, Command};
use spotify::SpotifyWrapper;

pub struct Commander {
    pub spotify: SpotifyWrapper
}

impl Commander {
    pub fn handle(&self, command: Command) -> Option<String> {
        match command {
            Command::Add(command) => self.add_song_to_playlist(command),
            Command::Error => None,
            Command::List => self.printable_list(),
        }
    }

    fn printable_list(&self) -> Option<String> {
        let result = self.spotify.songs_in_playlist();

        if let Some(list) = result {
            let names = list.iter().map(|item| item.name.clone()).collect::<Vec<_>>();
            Some(names.join("\n"))
        } else {
            println!("{:?}", result);
            None
        }
    }

    fn add_song_to_playlist(&self, add: AddCommand) -> Option<String> {
        let result = self.spotify.fetch_song(&add.uri);

        if let Some(song) = result {
            if let Some(_) = self.spotify.add_song_to_playlist(&song.uri) {
                Some("Song successfully added!".to_owned())
            } else {
                Some("Unable to add song :-(".to_owned())
            }
        } else {
            Some("Could not find song!".to_owned())
        }
    }
}