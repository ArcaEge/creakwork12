use industrial_io::{self as iio, Channel, ChannelType::Angl};

pub struct Hinge {
    channel: Channel,
}

impl Hinge {
    pub fn new() -> Self {
        let iio_ctx = iio::Context::new().expect("create iio context");
        let devices: Vec<_> = iio_ctx.devices().collect();

        Self {
            channel: devices
                .iter()
                .flat_map(|device| device.channels())
                .find(|channel| channel.channel_type() == Angl && channel.has_attr("raw"))
                .expect("get angle sensor channel"),
        }
    }

    pub fn get_reading(&self) -> i64 {
        self.channel
            .attr_read_int("raw")
            .expect("read hinge sensor")
    }
}
