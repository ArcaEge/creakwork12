use rodio::{Decoder, DeviceSinkBuilder, Player, Source};
use std::io::Cursor;
use std::thread::sleep;

fn main() {
    let mut handle = DeviceSinkBuilder::open_default_sink().expect("open default audio sink");
    handle.log_on_drop(false);

    let player = Player::connect_new(&handle.mixer());

    let creak_file = include_bytes!("../assets/creak.wav");
    let creak_cursor = Cursor::new(creak_file);
    let creak = Decoder::builder()
        .with_data(creak_cursor)
        .with_hint("wav")
        .with_gapless(true)
        .build()
        .expect("decode creak")
        .repeat_infinite();

    player.append(creak);
    // player.pause();

    sleep(std::time::Duration::from_millis(5000));
    player.pause();
    sleep(std::time::Duration::from_millis(500));
    player.play();
    sleep(std::time::Duration::from_millis(5000));
}
