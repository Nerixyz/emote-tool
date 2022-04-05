mod decoders;
pub mod formats;
pub mod frames;
pub mod types;

use crate::ffmpeg::{
    decoders::open_decoder,
    formats::AcceptedFormats,
    types::{FrameData, TimingData},
};
use crossbeam::channel::Sender;
use ffmpeg_next::{format, format::Pixel, frame, media::Type, software::scaling};
use indicatif::ProgressBar;
use std::{iter, path::Path};

#[derive(Debug, thiserror::Error)]
pub enum FfmpegError {
    #[error("Ffmpeg error: {0}")]
    Ffmpeg(#[from] ffmpeg_next::Error),
    #[error("Cannot select pixel format to convert frames to - got source format {0:?}")]
    NoPixelFormat(Pixel),
    #[error("Cannot send frame to task thread (channel-full: {0})")]
    SendFrame(bool),
    #[error("Stream had no timing information")]
    NoTimingInformation,
}

pub fn read_initial_stream<P: AsRef<Path>>(
    input: &P,
) -> Result<(format::context::Input, usize), FfmpegError> {
    let ctx = format::input(input)?;
    let istream = ctx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;

    let stream_idx = istream.index();
    Ok((ctx, stream_idx))
}

pub fn emit_frames(
    mut input_ctx: format::context::Input,
    istream_idx: usize,
    accepted_formats: AcceptedFormats,
    frame_tx: Sender<FrameData>,
    progress: ProgressBar,
) -> Result<(), FfmpegError> {
    let istream = input_ctx.stream(istream_idx).unwrap();
    let stream_time_base = istream.time_base();

    progress.set_length(istream.frames() as u64);

    let mut decoder = open_decoder(&istream)?;

    let mut scaler = if accepted_formats.passes(decoder.format()) {
        None
    } else {
        Some(scaling::context::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            accepted_formats
                .select(decoder.format())
                .ok_or_else(|| FfmpegError::NoPixelFormat(decoder.format()))?,
            decoder.width(),
            decoder.height(),
            scaling::Flags::BILINEAR,
        )?)
    };

    let mut decoded = frame::Video::empty();
    for pack in input_ctx.packets().map(Some).chain(iter::once(None)) {
        match pack {
            Some((stream, packet)) if stream.index() == istream_idx => {
                decoder.send_packet(&packet)?;
            }
            Some(_) => continue,
            None => decoder.send_eof()?,
        };

        while decoder.receive_frame(&mut decoded).is_ok() {
            let timing = TimingData::try_new(&decoded, stream_time_base)
                .ok_or(FfmpegError::NoTimingInformation)?;
            let frame = match &mut scaler {
                Some(scaler) => {
                    let mut frame = frame::Video::empty();
                    scaler.run(&decoded, &mut frame)?;
                    frame
                }
                None => std::mem::replace(&mut decoded, frame::Video::empty()),
            };
            frame_tx
                .send((frame, timing))
                .map_err(|_| FfmpegError::SendFrame(frame_tx.is_full()))?;
            progress.inc(1);
        }
    }

    progress.finish_and_clear();
    Ok(())
}
