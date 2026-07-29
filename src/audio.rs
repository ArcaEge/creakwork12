use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::io::Cursor;

pub struct Audio {
    player: Player,
    _handle: MixerDeviceSink,
}

impl Audio {
    pub fn new() -> Self {
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
        player.pause();

        Self {
            player,
            _handle: handle,
        }
    }

    pub fn play(&self) {
        self.player.play();
    }

    pub fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    pub fn pause(&self) {
        self.player.pause();
    }
}
