use std::{thread::sleep, time::Duration};

mod audio;
mod hinge;

const CREAK_THRESHOLD: u64 = 1;

fn main() -> ! {
    let h = hinge::Hinge::new();
    let a = audio::Audio::new();

    let mut last_reading = h.get_reading();

    loop {
        sleep(Duration::from_millis(100));

        let reading = h.get_reading();
        let diff = reading.abs_diff(last_reading);
        // println!("")

        if diff >= CREAK_THRESHOLD && a.is_paused() {
            a.play();
        } else {
            a.pause();
        }

        last_reading = reading;
    }

    // sleep(std::time::Duration::from_millis(5000));
    // player.pause();
    // sleep(std::time::Duration::from_millis(500));
    // player.play();
    // sleep(std::time::Duration::from_millis(5000));
}
