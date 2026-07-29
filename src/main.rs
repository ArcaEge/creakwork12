use one_euro_rs::OneEuroFilter;
use std::{thread::sleep, time::Duration};

mod audio;
mod hinge;

const CREAK_HIGH_THRESHOLD: f64 = 0.015;
const CREAK_LOW_THRESHOLD: f64 = 0.001;
const DELAY: u64 = 50;
const FREQUENCY: f64 = 1000.0 / DELAY as f64;
const CUTOFF_MIN: f64 = 1.0;
const CUTOFF_D: f64 = 0.4;
const BETA: f64 = 0.001;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to creakwork12");
    let rec = rerun::RecordingStreamBuilder::new("angle").spawn()?;

    let h = hinge::Hinge::new();
    let a = audio::Audio::new();
    let mut one_euro = OneEuroFilter::new(FREQUENCY, CUTOFF_MIN, CUTOFF_D, BETA);

    let mut last_reading = h.get_reading();

    loop {
        sleep(Duration::from_millis(DELAY));

        let mut reading = h.get_reading();

        if reading > 360 || reading < 0 {
            reading = last_reading;
        }

        let diff = reading as f64 - last_reading as f64;
        let ddiff = diff / DELAY as f64;
        let filtered_ddiff = one_euro.filter(ddiff);

        rec.log("diff/delay", &rerun::Scalars::new([ddiff]))
            .unwrap();
        rec.log("filtered_ddiff", &rerun::Scalars::new([filtered_ddiff]))
            .unwrap();

        if filtered_ddiff.abs() >= CREAK_HIGH_THRESHOLD {
            a.play();
        } else {
            a.pause();
        }

        last_reading = reading;
    }
}
