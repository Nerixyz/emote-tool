use crate::ffmpeg::types::FrameData;
use crossbeam::channel::Receiver;
use ffmpeg_next::{format, format::context};
use indicatif::ProgressBar;
use std::{io, path::PathBuf};

pub trait EncoderTask {
    type CliArgs;
    type Config: Send + 'static;
    type ConfigError: std::error::Error;
    type RunError: std::error::Error + Send + 'static;
    type EncoderStats: std::fmt::Display + Send + 'static;

    fn accepted_formats() -> &'static [format::Pixel];
    fn accepted_alpha_formats() -> &'static [format::Pixel];

    fn make_output_path(output_name: &str) -> PathBuf;
    fn configure(
        args: Self::CliArgs,
        stream: &ffmpeg_next::Stream,
        ctx: &context::Input,
    ) -> Result<Self::Config, Self::ConfigError>;
    fn run_animation<W: io::Write + 'static>(
        output: W,
        config: Self::Config,
        frame_rx: Receiver<FrameData>,
        progress: ProgressBar,
    ) -> Result<Self::EncoderStats, Self::RunError>;
    fn run_still<W: io::Write + 'static>(
        output: W,
        config: Self::Config,
        frame_rx: Receiver<FrameData>,
        progress: ProgressBar,
    ) -> Result<Self::EncoderStats, Self::RunError>;
}
