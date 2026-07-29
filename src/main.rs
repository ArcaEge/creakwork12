use one_euro_rs::OneEuroFilter;
use std::{thread::sleep, time::Duration, u64};

mod audio;
mod hinge;

const CREAK_HIGH_THRESHOLD: f64 = 0.02;
const CREAK_LOW_THRESHOLD: f64 = 0.002;
const DELAY: u64 = 75;
const FREQUENCY: f64 = 1000.0 / DELAY as f64;
const CUTOFF_MIN: f64 = 0.8;
const CUTOFF_D: f64 = 0.0;
const BETA: f64 = 0.001;
const STATE_DELAY: u64 = 1;

const AUDIO_LOWEST_SPEED: f32 = 1.0;
const AUDIO_LOWEST_SPEED_AT: f64 = CREAK_LOW_THRESHOLD;
const AUDIO_HIGHEST_SPEED: f32 = 1.3;
const AUDIO_HIGHEST_SPEED_AT: f64 = 0.4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("welcome to creakwork12");
    let rec = rerun::RecordingStreamBuilder::new("angle").spawn()?;

    let h = hinge::Hinge::new();
    let a = audio::Audio::new();
    let mut one_euro = OneEuroFilter::new(FREQUENCY, CUTOFF_MIN, CUTOFF_D, BETA);

    let mut last_reading = h.get_reading();
    let mut state = State::Inactive;
    let mut last_state_changed = STATE_DELAY;

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

        match state {
            State::Inactive => {
                if filtered_ddiff.abs() >= CREAK_HIGH_THRESHOLD {
                    if filtered_ddiff > 0.0 {
                        state = State::Positive;
                    } else {
                        state = State::Negative;
                    }

                    last_state_changed = 0;
                }
            }
            State::Positive => {
                if filtered_ddiff < CREAK_LOW_THRESHOLD {
                    state = State::Inactive;
                    last_state_changed = STATE_DELAY;
                }
            }
            State::Negative => {
                if -filtered_ddiff < CREAK_LOW_THRESHOLD {
                    state = State::Inactive;
                    last_state_changed = STATE_DELAY;
                }
            }
        }

        if last_state_changed >= STATE_DELAY {
            if let State::Inactive = state {
                a.pause();
            } else {
                a.play();
                a.set_speed(diff_to_speed(filtered_ddiff));
            }
        }

        last_reading = reading;
        last_state_changed += 1;
    }
}

pub fn diff_to_speed(diff: f64) -> f32 {
    ((diff
        .abs()
        .clamp(AUDIO_LOWEST_SPEED_AT, AUDIO_HIGHEST_SPEED_AT)
        - AUDIO_LOWEST_SPEED_AT)
        / (AUDIO_HIGHEST_SPEED_AT - AUDIO_LOWEST_SPEED_AT)) as f32
        * (AUDIO_HIGHEST_SPEED - AUDIO_LOWEST_SPEED)
        + AUDIO_LOWEST_SPEED
}

#[derive(Debug)]
enum State {
    Inactive,
    Positive,
    Negative,
}
