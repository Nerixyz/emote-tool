use crate::EncoderTask;
use ffmpeg_next::format::Pixel;

pub struct AcceptedFormats {
    pub regular: &'static [Pixel],
    pub alpha: &'static [Pixel],
}

impl AcceptedFormats {
    pub fn for_task<T: EncoderTask>() -> Self {
        Self {
            regular: T::accepted_formats(),
            alpha: T::accepted_alpha_formats(),
        }
    }

    pub fn passes(&self, fmt: Pixel) -> bool {
        self.regular.contains(&fmt) || self.alpha.contains(&fmt)
    }

    pub fn select(&self, src_format: Pixel) -> Option<Pixel> {
        if is_alpha_format(src_format) {
            self.alpha.get(0).copied()
        } else {
            self.regular.get(0).copied()
        }
    }
}

fn is_alpha_format(format: Pixel) -> bool {
    matches!(
        format,
        Pixel::ARGB
            | Pixel::ABGR
            | Pixel::BGRA
            | Pixel::RGBA
            | Pixel::YUVA444P
            | Pixel::YUVA420P
            | Pixel::YUVA422P
    )
}
