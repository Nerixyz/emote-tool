use libavif_sys as sys;
use libavif_sys::avifCodecChoice;

#[derive(Debug, Copy, Clone, Eq, PartialEq, clap::ArgEnum)]
pub enum Codec {
    Auto = sys::AVIF_CODEC_CHOICE_AUTO as isize,
    Aom = sys::AVIF_CODEC_CHOICE_AOM as isize,
    Dav1d = sys::AVIF_CODEC_CHOICE_DAV1D as isize,
    LibGav1 = sys::AVIF_CODEC_CHOICE_LIBGAV1 as isize,
    Rav1e = sys::AVIF_CODEC_CHOICE_RAV1E as isize,
    Svt = sys::AVIF_CODEC_CHOICE_SVT as isize,
}

impl From<Codec> for sys::avifCodecChoice {
    fn from(codec: Codec) -> Self {
        codec as Self
    }
}

impl From<sys::avifCodecChoice> for Codec {
    fn from(choice: avifCodecChoice) -> Self {
        match choice {
            sys::AVIF_CODEC_CHOICE_AUTO => Self::Auto,
            sys::AVIF_CODEC_CHOICE_AOM => Self::Aom,
            sys::AVIF_CODEC_CHOICE_DAV1D => Self::Dav1d,
            sys::AVIF_CODEC_CHOICE_LIBGAV1 => Self::LibGav1,
            sys::AVIF_CODEC_CHOICE_RAV1E => Self::Rav1e,
            sys::AVIF_CODEC_CHOICE_SVT => Self::Svt,
            _ => Self::Auto,
        }
    }
}
