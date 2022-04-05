use libwebp_sys as sys;
use std::os::raw::c_int;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, clap::ArgEnum)]
pub enum ImageHint {
    Default = 0,
    Picture = 1,
    Photo = 2,
    Graph = 3,
    Last = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, clap::ArgEnum)]
pub enum AlphaFiltering {
    None = 0,
    Fast = 1,
    Best = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, clap::ArgEnum)]
pub enum PreprocessingFilter {
    None = 0,
    SegmentSmooth = 1,
    PseudoRandomDithering = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, clap::ArgEnum)]
pub enum Preset {
    Default,
    Picture,
    Photo,
    Drawing,
    Icon,
    Text,
}

impl From<sys::WebPImageHint> for ImageHint {
    fn from(hint: sys::WebPImageHint) -> Self {
        match hint {
            sys::WebPImageHint::WEBP_HINT_DEFAULT => Self::Default,
            sys::WebPImageHint::WEBP_HINT_PICTURE => Self::Picture,
            sys::WebPImageHint::WEBP_HINT_PHOTO => Self::Photo,
            sys::WebPImageHint::WEBP_HINT_GRAPH => Self::Graph,
            sys::WebPImageHint::WEBP_HINT_LAST => Self::Last,
        }
    }
}

impl From<ImageHint> for sys::WebPImageHint {
    fn from(hint: ImageHint) -> Self {
        match hint {
            ImageHint::Default => sys::WebPImageHint::WEBP_HINT_DEFAULT,
            ImageHint::Picture => sys::WebPImageHint::WEBP_HINT_PICTURE,
            ImageHint::Photo => sys::WebPImageHint::WEBP_HINT_PHOTO,
            ImageHint::Graph => sys::WebPImageHint::WEBP_HINT_GRAPH,
            ImageHint::Last => sys::WebPImageHint::WEBP_HINT_LAST,
        }
    }
}

impl From<c_int> for AlphaFiltering {
    fn from(flt: c_int) -> Self {
        match flt {
            0 => Self::None,
            1 => Self::Fast,
            2 => Self::Best,
            _ => Self::Fast,
        }
    }
}

impl From<AlphaFiltering> for c_int {
    fn from(flt: AlphaFiltering) -> Self {
        flt as u32 as _
    }
}

impl From<c_int> for PreprocessingFilter {
    fn from(flt: c_int) -> Self {
        match flt {
            0 => Self::None,
            1 => Self::SegmentSmooth,
            2 => Self::PseudoRandomDithering,
            _ => Self::None,
        }
    }
}

impl From<PreprocessingFilter> for c_int {
    fn from(flt: PreprocessingFilter) -> Self {
        flt as u32 as _
    }
}

impl From<Preset> for sys::WebPPreset {
    fn from(p: Preset) -> Self {
        match p {
            Preset::Default => Self::WEBP_PRESET_DEFAULT,
            Preset::Picture => Self::WEBP_PRESET_PICTURE,
            Preset::Photo => Self::WEBP_PRESET_PHOTO,
            Preset::Drawing => Self::WEBP_PRESET_DRAWING,
            Preset::Icon => Self::WEBP_PRESET_ICON,
            Preset::Text => Self::WEBP_PRESET_TEXT,
        }
    }
}
