use std::{thread::sleep, time::Duration};

mod audio;
mod hinge;

const CREAK_RAW_THRESHOLD: u64 = 4;
const CREAK_AVERAGE_THRESHOLD: f64 = 3.0;
const SMOOTHING_AMOUNT: f64 = 0.65;

fn main() -> ! {
    println!("Welcome to creakwork12");

    let h = hinge::Hinge::new();
    let a = audio::Audio::new();

    let mut last_reading = h.get_reading();
    a.play();

    // exponential moving average
    let mut avg_diff = 0.0;

    let mut last_state = false;

    loop {
        sleep(Duration::from_millis(120));

        let reading = h.get_reading();
        let diff = reading.abs_diff(last_reading);
        let raw_diff = reading - last_reading;
        avg_diff = avg_diff * SMOOTHING_AMOUNT + raw_diff as f64 * (1.0 - SMOOTHING_AMOUNT);

        if diff <= 1 {
            avg_diff = 0.0;
        }

        println!("diff: {:?}, avg_diff: {:.2}", raw_diff, avg_diff);

        if avg_diff.abs() >= CREAK_AVERAGE_THRESHOLD {
            if true {
                a.play();
            }

            last_state = true;
        } else {
            // if !last_state {
            a.pause();
            // }

            last_state = false;
        }

        last_reading = reading;
    }

    // sleep(std::time::Duration::from_millis(5000));
    // player.pause();
    // sleep(std::time::Duration::from_millis(500));
    // player.play();
    // sleep(std::time::Duration::from_millis(5000));
}
