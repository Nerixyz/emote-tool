use crate::{apply_options, cli, getter_debug};
use libwebp_sys as sys;
use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
    os,
    os::raw::c_int,
    str::FromStr,
};

pub struct AnimEncoderOptions {
    opts: sys::WebPAnimEncoderOptions,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeyframeDistance {
    MinMax(u32, u32),
    Disabled,
    AllFrames,
}

pub type Argb = [u8; 4];

impl AnimEncoderOptions {
    pub fn new() -> Option<Self> {
        unsafe {
            let mut opts = std::mem::MaybeUninit::uninit();
            if sys::WebPAnimEncoderOptionsInitInternal(opts.as_mut_ptr(), sys::WEBP_MUX_ABI_VERSION)
                != 0
            {
                Some(Self {
                    opts: opts.assume_init(),
                })
            } else {
                None
            }
        }
    }

    pub fn minimize_size(&self) -> bool {
        self.opts.minimize_size != 0
    }

    /// If true, minimize the output size (slow). Implicitly disables key-frame insertion.
    pub fn set_minimize_size(&mut self, minimize_size: bool) -> &mut Self {
        self.opts.minimize_size = if minimize_size { 1 } else { 0 };
        self
    }

    pub fn keyframe_distance(&self) -> KeyframeDistance {
        (self.opts.kmin, self.opts.kmax).into()
    }

    pub fn set_keyframe_distance(&mut self, distance: KeyframeDistance) -> &mut Self {
        let (min, max) = distance.into();
        self.opts.kmin = min;
        self.opts.kmax = max;
        self
    }

    pub fn allow_mixed(&self) -> bool {
        self.opts.allow_mixed != 0
    }

    pub fn set_allow_mixed(&mut self, allow_mixed: bool) -> &mut Self {
        self.opts.allow_mixed = if allow_mixed { 1 } else { 0 };
        self
    }

    pub fn background_color(&self) -> Argb {
        self.opts.anim_params.bgcolor.to_be_bytes()
    }

    pub fn set_background_color(&mut self, color: Argb) -> &mut Self {
        self.opts.anim_params.bgcolor = u32::from_be_bytes(color);
        self
    }

    /// 0 = infinite
    pub fn loop_count(&self) -> i32 {
        self.opts.anim_params.loop_count as _
    }

    /// 0 = infinite
    pub fn set_loop_count(&mut self, loop_count: i32) -> &mut Self {
        self.opts.anim_params.loop_count = loop_count as _;
        self
    }

    pub fn apply_cli_options(&mut self, opts: &cli::WebpOptions) {
        apply_options!(self, opts;
            minimize_size,
            keyframe_distance,
            allow_mixed,
            loop_count,
        );
        if let Some(color) = &opts.background_color {
            self.set_background_color(color.0);
        }
    }

    pub(super) fn as_ptr(&self) -> *const sys::WebPAnimEncoderOptions {
        &self.opts
    }
}

impl Debug for AnimEncoderOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        getter_debug!(
            self, f, "AnimEncoderOptions";
            minimize_size,
            keyframe_distance,
            allow_mixed,
            loop_count,
            background_color,
        )
    }
}

impl From<(os::raw::c_int, os::raw::c_int)> for KeyframeDistance {
    fn from((min, max): (c_int, c_int)) -> Self {
        if max <= 0 {
            Self::Disabled
        } else if max == 1 {
            Self::AllFrames
        } else {
            Self::MinMax(min as u32, max as u32)
        }
    }
}

impl From<KeyframeDistance> for (os::raw::c_int, os::raw::c_int) {
    fn from(dst: KeyframeDistance) -> Self {
        match dst {
            KeyframeDistance::MinMax(min, max) => (min as _, max as _),
            KeyframeDistance::Disabled => (-1, 0),
            KeyframeDistance::AllFrames => (0, 1),
        }
    }
}

impl FromStr for KeyframeDistance {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "disabled" => Ok(Self::Disabled),
            "allframes" | "all-frames" | "allFrames" | "all_frames" => Ok(Self::AllFrames),
            s if s.contains("..") => {
                let mut iter = s.split("..").filter_map(|s| s.trim().parse::<u32>().ok());
                Ok(Self::MinMax(
                    iter.next().ok_or("no min")?,
                    iter.next().ok_or("no max")?,
                ))
            }
            s if s.contains(',') => {
                let mut iter = s.split(',').filter_map(|s| s.trim().parse::<u32>().ok());
                Ok(Self::MinMax(
                    iter.next().ok_or("no min")?,
                    iter.next().ok_or("no max")?,
                ))
            }
            _ => Err("Invalid distance, try 'disabled', 'all-frames', or '3..5'".into()),
        }
    }
}
