use libavif_sys as sys;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum YuvFormat {
    Yuv400 = sys::AVIF_PIXEL_FORMAT_YUV400 as isize,
    Yuv420 = sys::AVIF_PIXEL_FORMAT_YUV420 as isize,
    Yuv422 = sys::AVIF_PIXEL_FORMAT_YUV422 as isize,
    Yuv444 = sys::AVIF_PIXEL_FORMAT_YUV444 as isize,
}

impl From<YuvFormat> for sys::avifPixelFormat {
    fn from(fmt: YuvFormat) -> Self {
        fmt as sys::avifPixelFormat
    }
}
