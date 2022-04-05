use crate::{avif, webp};
use clap::{Args, Parser, Subcommand};
use hex::FromHex;
use std::{borrow::Cow, os::raw::c_int, str::FromStr};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    Avif(AvifCommand),
    Webp(WebpCommand),
}

#[derive(Args)]
pub struct IoOptions {
    pub input: String,
    #[clap(default_value = "out")]
    pub output: String,
}

#[derive(Args)]
pub struct AvifCommand {
    #[clap(flatten)]
    pub io: IoOptions,
    #[clap(flatten)]
    pub opts: AvifOptions,
}

#[derive(Args)]
pub struct WebpCommand {
    #[clap(flatten)]
    pub io: IoOptions,
    #[clap(flatten)]
    pub opts: WebpOptions,
}

#[derive(Args)]
pub struct AvifOptions {
    #[clap(short, long, arg_enum, default_value_t = avif::Codec::Auto)]
    pub codec: avif::Codec,
    #[clap(long, default_value = "0")]
    pub quantizer: u8,
    #[clap(long, default_value = "0")]
    pub quantizer_alpha: u8,
    #[clap(long, default_value = "10")]
    pub speed: u8,
    #[clap(long)]
    pub max_threads: Option<usize>,
}

#[derive(Args)]
pub struct WebpOptions {
    // general encoder options
    #[clap(long, arg_enum)]
    pub preset: Option<webp::Preset>,

    #[clap(long)]
    pub lossless: Option<bool>,
    #[clap(long)]
    pub quality: Option<f32>,
    #[clap(long)]
    pub method: Option<c_int>,
    #[clap(long, arg_enum)]
    pub image_hint: Option<webp::ImageHint>,
    #[clap(long)]
    pub target_size: Option<c_int>,
    #[clap(long)]
    pub target_psnr: Option<f32>,
    #[clap(long)]
    pub segments: Option<c_int>,
    #[clap(long)]
    pub sns_strength: Option<c_int>,
    #[clap(long)]
    pub filter_strength: Option<c_int>,
    #[clap(long)]
    pub filter_sharpness: Option<c_int>,
    #[clap(long)]
    pub strong_filter: Option<bool>,
    #[clap(long)]
    pub autofilter: Option<bool>,
    #[clap(long)]
    pub alpha_compression: Option<bool>,
    #[clap(long, arg_enum)]
    pub alpha_filtering: Option<webp::AlphaFiltering>,
    #[clap(long)]
    pub alpha_quality: Option<c_int>,
    #[clap(long)]
    pub pass: Option<c_int>,
    #[clap(long)]
    pub show_compressed: Option<bool>,
    #[clap(long, arg_enum)]
    pub preprocessing: Option<webp::PreprocessingFilter>,
    #[clap(long)]
    pub partitions: Option<c_int>,
    #[clap(long)]
    pub partition_limit: Option<c_int>,
    #[clap(long)]
    pub emulate_jpeg_size: Option<bool>,
    #[clap(long)]
    pub thread_level: Option<bool>,
    #[clap(long)]
    pub low_memory: Option<bool>,
    #[clap(long)]
    pub near_lossless: Option<c_int>,
    #[clap(long)]
    pub exact: Option<bool>,
    #[clap(long)]
    pub use_delta_palette: Option<bool>,
    #[clap(long)]
    pub use_sharp_yuv: Option<bool>,

    //anim encoder options
    #[clap(long)]
    pub minimize_size: Option<bool>,
    #[clap(long)]
    pub keyframe_distance: Option<webp::KeyframeDistance>,
    #[clap(long)]
    pub allow_mixed: Option<bool>,
    #[clap(long)]
    pub background_color: Option<BackgroundColor>,
    #[clap(long)]
    pub loop_count: Option<i32>,
}

pub struct BackgroundColor(pub webp::ARGB);

impl FromStr for BackgroundColor {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            match s.len() {
                7 => {
                    let rgb = <[u8; 3]>::from_hex(&s[1..]).map_err(|e| e.to_string())?;
                    Ok(Self([255, rgb[0], rgb[1], rgb[2]]))
                }
                9 => {
                    let argb = <[u8; 4]>::from_hex(&s[1..]).map_err(|e| e.to_string())?;
                    Ok(Self(argb))
                }
                _ => Err("Expected #abcdef, or #abcdef01".into()),
            }
        } else {
            Err("Expected #abcdef, or #abcdef01".into())
        }
    }
}
#[macro_export]
macro_rules! apply_options {
    ($self:ident, $opts:ident; $($opt:ident,)*) => {
            $(if let Some(v) = &$opts.$opt {
                concat_idents::concat_idents!(fn_name = set_, $opt {
                    $self.fn_name(*v);
                });
            })*
    };
}
