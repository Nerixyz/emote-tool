use ffmpeg_next::{frame, Rational};

pub type FrameData = (frame::Video, TimingData);

#[derive(Debug)]
pub struct TimingData {
    /// in time_base
    pub timestamp: i64,
    pub time_base: Rational,
}

impl TimingData {
    pub fn try_new(frame: &frame::Video, stream_time_base: Rational) -> Option<Self> {
        Some(Self {
            timestamp: frame.timestamp()?,
            time_base: stream_time_base,
        })
    }

    pub fn ts_in_ms(&self) -> i64 {
        (1000 * self.timestamp * self.time_base.0 as i64) / (self.time_base.1 as i64)
    }
}
