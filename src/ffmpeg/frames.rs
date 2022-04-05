use ffmpeg_next::{
    ffi::AV_TIME_BASE,
    format::{context, stream},
};

pub fn extract_frames(stream: &stream::Stream, ctx: &context::Input) -> i64 {
    if stream.frames() > 0 {
        stream.frames()
    } else if ctx.duration() > 0 {
        let rate = stream.avg_frame_rate();
        (ctx.duration() * rate.0 as i64) / (AV_TIME_BASE as i64 * rate.1 as i64)
    } else {
        0
    }
}

pub fn extract_duration_ms(stream: &stream::Stream, ctx: &context::Input) -> i64 {
    let tb = stream.time_base();
    let duration_in_tb = stream.duration();

    if duration_in_tb > 0 {
        (1000 * duration_in_tb * tb.0 as i64) / (tb.1 as i64)
    } else {
        (ctx.duration() * 1000) / (AV_TIME_BASE as i64)
    }
}
