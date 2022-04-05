use crate::{
    cli,
    ffmpeg::{frames::extract_duration_ms, types::FrameData},
    webp::{
        anim_encoder::AnimEncoder, config::Config, image::WebpImage, image_encode::encode_image,
        options::AnimEncoderOptions, AnimEncoderError, CreateImageError, Preset, StillEncoderError,
    },
    EncoderTask,
};
use crossbeam::channel::{Receiver, RecvError};
use ffmpeg_next::{
    format::{context, Pixel},
    Stream,
};
use indicatif::ProgressBar;
use libwebp_sys as sys;
use std::{
    fmt::{Display, Formatter, Pointer},
    io,
    io::Write,
    path::PathBuf,
    ptr::NonNull,
};

pub struct WebpEncoderTask;

pub struct WebpEncoderConfig {
    args: cli::WebpOptions,
    width: usize,
    height: usize,
    duration_ms: i32,
}

pub enum WebpEncoderStats {
    Still(Option<sys::WebPAuxStats>),
    Animation(usize),
}

#[derive(Debug, thiserror::Error)]
pub enum WebpEncoderError {
    #[error("Still Encoder errored: {0}")]
    StillEncoderError(#[from] StillEncoderError),
    #[error("Anim Encoder errored: {0}")]
    AnimEncoderError(#[from] AnimEncoderError),
    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Encoder thread didn't receive any image ({0})")]
    NoImageReceived(#[from] RecvError),
    #[error("Couldn't create a libwebp picture from a libav frame: {0}")]
    CreateImageError(#[from] CreateImageError),
    #[error("Couldn't create config")]
    CreateConfigError,
    #[error("Couldn't create anim encoder options")]
    CreateAnimEncoderOptions,
    #[error("Couldn't create anim encoder")]
    CreateAnimEncoder,
    #[error("Invalid config, validation failed")]
    InvalidConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum WebpEncoderConfigError {
    #[error("AVStream didn't have 'codecpar' so width and height couldn't be read.")]
    NoCodecPar,
}

impl EncoderTask for WebpEncoderTask {
    type CliArgs = cli::WebpOptions;
    type Config = WebpEncoderConfig;
    type ConfigError = WebpEncoderConfigError;
    type RunError = WebpEncoderError;
    type EncoderStats = WebpEncoderStats;

    fn accepted_formats() -> &'static [Pixel] {
        &[Pixel::YUV420P]
    }

    fn accepted_alpha_formats() -> &'static [Pixel] {
        &[Pixel::YUVA420P]
    }

    fn make_output_path(output_name: &str) -> PathBuf {
        PathBuf::from(output_name).with_extension("webp")
    }

    fn configure(
        args: Self::CliArgs,
        stream: &Stream,
        ctx: &context::Input,
    ) -> Result<Self::Config, Self::ConfigError> {
        let (width, height) = unsafe {
            let par =
                NonNull::new((*stream.as_ptr()).codecpar).ok_or(Self::ConfigError::NoCodecPar)?;
            (par.as_ref().width, par.as_ref().height)
        };
        Ok(Self::Config {
            args,
            width: width as usize,
            height: height as usize,
            duration_ms: extract_duration_ms(stream, ctx) as i32,
        })
    }

    fn run_animation<W: Write>(
        mut output: W,
        config: Self::Config,
        frame_rx: Receiver<FrameData>,
        progress: ProgressBar,
    ) -> Result<Self::EncoderStats, Self::RunError> {
        let encoder_config = encoder_config_from_cli(&config.args)?;
        let mut anim_encoder_opts =
            AnimEncoderOptions::new().ok_or(Self::RunError::CreateAnimEncoderOptions)?;
        anim_encoder_opts.apply_cli_options(&config.args);
        let mut encoder = AnimEncoder::create(
            config.width,
            config.height,
            &anim_encoder_opts,
            encoder_config,
        )
        .ok_or(Self::RunError::CreateAnimEncoder)?;

        for (mut frame, timing) in frame_rx {
            let mut image = WebpImage::from_av_frame(&mut frame)?;
            encoder.add_image(&mut image, timing.ts_in_ms() as i32)?;
            progress.inc(1);
        }
        let data = encoder.finalize(config.duration_ms)?;
        output.write_all(data.as_slice())?;
        progress.finish_and_clear();

        Ok(WebpEncoderStats::Animation(data.len()))
    }

    fn run_still<W: Write + 'static>(
        output: W,
        config: Self::Config,
        frame_rx: Receiver<FrameData>,
        progress: ProgressBar,
    ) -> Result<Self::EncoderStats, Self::RunError> {
        let (mut frame, _) = frame_rx.recv()?;
        let mut image = WebpImage::from_av_frame(&mut frame)?;
        let encoder_config = encoder_config_from_cli(&config.args)?;
        let stats = encode_image(&mut image, output, &encoder_config)?;
        progress.finish_and_clear();
        Ok(WebpEncoderStats::Still(stats))
    }
}

impl Display for WebpEncoderStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            WebpEncoderStats::Still(Some(stats)) => stats.fmt(f),
            WebpEncoderStats::Still(None) => write!(f, "No stats"),
            WebpEncoderStats::Animation(bytes) => write!(f, "Written {bytes} bytes"),
        }
    }
}

fn encoder_config_from_cli(cli: &cli::WebpOptions) -> Result<Config, WebpEncoderError> {
    let mut encoder_config = Config::new(cli.preset.unwrap_or(Preset::Default))
        .ok_or(WebpEncoderError::CreateConfigError)?;
    encoder_config.apply_cli_options(cli);
    if encoder_config.validate() {
        Ok(encoder_config)
    } else {
        Err(WebpEncoderError::InvalidConfig)
    }
}
