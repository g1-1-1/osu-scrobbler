mod config;
mod last_fm;
mod osu;
mod scrobble_loop;

use scrobble_loop::main as scrobble_loop;

fn main() {
    println!("Started scrobbler!");
    scrobble_loop();
}
