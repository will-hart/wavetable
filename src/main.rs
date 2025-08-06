use std::time::Duration;

use audio::AudioSystem;

pub mod audio;
pub mod nodes;

fn main() {
    let mut audio = AudioSystem::new();

    for _ in 0..84_200 {
        audio.update();
        std::thread::sleep(Duration::from_micros(1_000_000 / 44_100));
    }
}
