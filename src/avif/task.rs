use super::image::BorrowedAvifImage;
use crate::{cli::AvifOptions, ffmpeg::types::FrameData, EncoderTask};
use crossbeam::channel::{Receiver, RecvError};
use ffmpeg_next::{
    format,
    format::{context, Pixel},
    Stream,
};
use indicatif::ProgressBar;
use std::{
    fmt::{Debug, Display, Formatter},
    io,
    io::Write,
    path::PathBuf,
};

pub struct AvifEncoderTask;

#[derive(Debug, thiserror::Error)]
pub enum AvifEncoderError {
    #[error("Encoder error: {0}")]
    Encoder(#[from] super::Error),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("A livav frame could not be converted to a libavif frame (format: {0:?}")]
    FrameConversion(format::Pixel),
    #[error("Cannot create encoder")]
    CannotCreateEncoder,
    #[error("Encoder thread didn't receive any image ({0})")]
    NoImageReceived(#[from] RecvError),
}

pub struct AvifEncoderConfig {
    args: AvifOptions,
    timescale: u64,
}

pub struct AvifEncoderStats {
    bytes_written: usize,
    encoder_data: String,
}

impl Display for AvifEncoderStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Written {} bytes ({})",
            self.bytes_written, self.encoder_data
        )
    }
}

impl EncoderTask for AvifEncoderTask {
    type CliArgs = AvifOptions;
    type Config = AvifEncoderConfig;
    type ConfigError = io::Error;
    type RunError = AvifEncoderError;
    type EncoderStats = AvifEncoderStats;

    fn accepted_formats() -> &'static [Pixel] {
        &[Pixel::YUV444P, Pixel::YUV420P, Pixel::YUV422P]
    }

    fn accepted_alpha_formats() -> &'static [Pixel] {
        &[Pixel::YUVA444P]
    }

    fn make_output_path(output_name: &str) -> PathBuf {
        PathBuf::from(output_name).with_extension("avif")
    }

    fn configure(
        args: Self::CliArgs,
        stream: &Stream,
        _ctx: &context::Input,
    ) -> Result<Self::Config, Self::ConfigError> {
        let timebase = stream.time_base();
        Ok(Self::Config {
            args,
            timescale: (timebase.1 / timebase.0) as u64,
        })
    }

    fn run_animation<W: Write>(
        mut output: W,
        config: Self::Config,
        frame_rx: Receiver<FrameData>,
        progress: ProgressBar,
    ) -> Result<Self::EncoderStats, Self::RunError> {
        let mut encoder = make_encoder(&config)?;

        for (mut frame, _) in frame_rx {
            let format = frame.format();
            let pkt_duration = unsafe { (*frame.as_ptr()).pkt_duration } as u64;
            let img = BorrowedAvifImage::from_ffmpeg(&mut frame)
                .ok_or(Self::RunError::FrameConversion(format))?;
            encoder.add_image_none(img.as_ref(), pkt_duration)?;
            progress.inc(1);
        }

        progress.finish_with_message("Finishing...");
        let data = encoder.finish()?;
        progress.finish_and_clear();
        output.write_all(data.as_slice())?;
        Ok(AvifEncoderStats {
            encoder_data: format!("{encoder:?}"),
            bytes_written: data.len(),
        })
    }

    fn run_still<W: Write>(
        mut output: W,
        config: Self::Config,
        frame_rx: Receiver<FrameData>,
        progress: ProgressBar,
    ) -> Result<Self::EncoderStats, Self::RunError> {
        let mut encoder = make_encoder(&config)?;

        let (mut frame, _) = frame_rx.recv()?;
        let format = frame.format();
        let img = BorrowedAvifImage::from_ffmpeg(&mut frame)
            .ok_or(Self::RunError::FrameConversion(format))?;

        let data = encoder.encode_single_image(img.as_ref())?;
        progress.finish_and_clear();
        output.write_all(data.as_slice())?;
        Ok(AvifEncoderStats {
            encoder_data: format!("{encoder:?}"),
            bytes_written: data.len(),
        })
    }
}

fn make_encoder(config: &AvifEncoderConfig) -> Result<super::Encoder, AvifEncoderError> {
    let mut encoder = super::Encoder::new().ok_or(AvifEncoderError::CannotCreateEncoder)?;
    encoder
        .set_timescale(config.timescale)
        .set_max_threads(config.args.max_threads.unwrap_or_else(|| num_cpus::get()))
        .set_speed(config.args.speed)
        .set_codec(config.args.codec)
        .set_quantizer(config.args.quantizer)
        .set_quantizer_alpha(config.args.quantizer_alpha);
    Ok(encoder)
}
