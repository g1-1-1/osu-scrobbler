use crate::config::get_config;
use crate::last_fm::LastfmScrobbler;
use crate::osu::{nerinyan::Beatmapset, window::OsuWindowDetails};
use crate::scrobble_loop::get_current_timestamp;

pub struct OsuScrobble {
    pub window_details: OsuWindowDetails,
    pub beatmapset: Beatmapset,
    end_timestamps: Vec<u64>,
}

impl OsuScrobble {
    pub fn new(window_details: &OsuWindowDetails, beatmapset: &Beatmapset) -> Self {
        let timestamp = get_current_timestamp();

        // Hide this for now. Need to figure out how to make the last now playing message disappear after scrobbling a track.
        /*
        let config = get_config();

        match scrobbler.set_now_playing(
            if config.scrobble.use_original_metadata {
                &beatmapset.title_unicode
            } else {
                &beatmapset.title
            },
            if config.scrobble.use_original_metadata {
                &beatmapset.artist_unicode
            } else {
                &beatmapset.artist
            },
            if config.scrobble.use_original_metadata {
                &beatmapset.title_unicode
            } else {
                &beatmapset.title
            },
        ) {
            Ok(_) => (),
            Err(err) => println!(
                "An error occurred while trying to set Last.fm now playing: {}",
                err
            ),
        }
        */

        Self {
            window_details: window_details.to_owned(),
            beatmapset: beatmapset.to_owned(),
            end_timestamps: vec![timestamp + (beatmapset.length / 2), timestamp + 240],
        }
    }

    pub fn end(&self, scrobbler: &LastfmScrobbler) {
        let timestamp = get_current_timestamp();

        if timestamp >= self.end_timestamps[0] || timestamp >= self.end_timestamps[1] {
            let config = get_config();

            match scrobbler.scrobble(
                if config.scrobble.use_original_metadata {
                    &self.beatmapset.title_unicode
                } else {
                    &self.beatmapset.title
                },
                if config.scrobble.use_original_metadata {
                    &self.beatmapset.artist_unicode
                } else {
                    &self.beatmapset.artist
                },
                if config.scrobble.use_original_metadata {
                    &self.beatmapset.title_unicode
                } else {
                    &self.beatmapset.title
                },
            ) {
                Ok(_) => println!("Scrobbled ^"),
                Err(err) => println!(
                    "An error occurred while trying to scrobble in Last.fm: {}",
                    err
                ),
            };
        } else {
            println!("Not scrobbled ^");
        }
    }
}
