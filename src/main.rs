use std::time::Duration;

fn main() {
    let mut audio = synthlib::audio::AudioSystem::new();

    for _ in 0..84_200 {
        audio.update();
        std::thread::sleep(Duration::from_micros(1_000_000 / 44_100));
    }
}
