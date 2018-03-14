extern crate rspotify;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::model::track::FullTrack;

use std::env;

pub struct SpotifyWrapper {
    client: rspotify::spotify::client::Spotify
}

impl SpotifyWrapper {
    pub fn new() -> SpotifyWrapper {
        let spotify_id = env::var("SPOTIFY_CLIENT_ID")
            .expect("Couldn't read environment variable SPOTIFY_CLIENT_ID");

        let spotify_secret = env::var("SPOTIFY_CLIENT_SECRET")
            .expect("Couldn't read environment variable SPOTIFY_CLIENT_SECRET");

        let spotify_uri = env::var("SPOTIFY_CLIENT_URI")
            .expect("Couldn't read environment variable SPOTIFY_CLIENT_URI");
        
        let mut oauth = SpotifyOAuth::default()
            .client_id(&spotify_id)
            .client_secret(&spotify_secret)
            .redirect_uri(&spotify_uri)
            .scope("playlist-modify-public")
            .build();
        
        match get_token(&mut oauth) {
            Some(token_info) => {
                let client_credentials = SpotifyClientCredentials::default()
                    .client_id(&spotify_id)
                    .client_secret(&spotify_secret)
                    .token_info(token_info)
                    .build();
                
                let client = Spotify::default()
                    .client_credentials_manager(client_credentials)
                    .build();
                
                SpotifyWrapper { client }
            }
            None => panic!("auth failed"),
        }
    }

    pub fn songs_in_playlist(&self) -> Option<Vec<FullTrack>> {
        let user_id = "thaunatos";
        let playlist_id = "5JaR88hsCmzoa3WjpT8jhf";

        let result = self.client.user_playlist_tracks(
            user_id, playlist_id, None, None, None, None
        );

        match result {
            Ok(page) => {
                Some(page.items
                    .iter()
                    .map(|item| item.clone().track)
                    .collect::<Vec<FullTrack>>()
                )
            },
            _ => None,
        }
    }

    pub fn fetch_song(&self, uri: &str) -> Option<FullTrack> {
        self.client.track(uri).ok()
    }

    pub fn add_song_to_playlist(&self, uri: &str) -> Option<()> {
        let user_id = "thaunatos";
        let playlist_id = "spotify:user:thaunatos:playlist:5JaR88hsCmzoa3WjpT8jhf";

        let mut tracks = vec![];
        tracks.push(uri.to_owned());

        let result = self.client.user_playlist_add_tracks(
            user_id, playlist_id, &tracks, None);

        match result {
            Ok(_) => Some(()),
            _ => None,
        }
    }
}
