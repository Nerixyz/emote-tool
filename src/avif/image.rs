use ffmpeg_next::{format::Pixel, frame};
use libavif_sys as sys;
use std::{marker::PhantomData, ptr::NonNull};

pub struct BorrowedAvifImage<'a> {
    inner: AvifImage,
    _marker: PhantomData<&'a ()>,
}

pub struct AvifImage {
    image: NonNull<sys::avifImage>,
}

impl AvifImage {
    pub(super) fn from_raw(image: NonNull<sys::avifImage>) -> Self {
        Self { image }
    }

    pub(super) fn inner(&self) -> *const sys::avifImage {
        self.image.as_ptr()
    }
}

impl<'a> BorrowedAvifImage<'a> {
    pub(super) fn from_raw(image: NonNull<sys::avifImage>) -> Self {
        Self {
            inner: AvifImage::from_raw(image),
            _marker: PhantomData::default(),
        }
    }

    pub fn from_ffmpeg(frame: &'a mut ffmpeg_next::frame::Video) -> Option<Self> {
        let format = frame.format();
        match format {
            Pixel::YUV420P | Pixel::YUV422P | Pixel::YUV444P => unsafe {
                assert_eq!(frame.planes(), 3);
                let mut image = create_raw_from_yuv_format(frame, format)?;

                for i in 0..3 {
                    image.as_mut().yuvPlanes[i] = frame.data_mut(i).as_mut_ptr();
                    image.as_mut().yuvRowBytes[i] = frame.stride(i) as u32;
                }

                image.as_mut().imageOwnsYUVPlanes = sys::AVIF_FALSE as sys::avifBool;
                Some(Self::from_raw(image))
            },
            Pixel::YUVA444P => unsafe {
                assert_eq!(frame.planes(), 4);
                let mut image = NonNull::new(sys::avifImageCreate(
                    frame.width() as i32,
                    frame.height() as i32,
                    8,
                    sys::AVIF_PIXEL_FORMAT_YUV444,
                ))?;

                for i in 0..3 {
                    image.as_mut().yuvPlanes[i] = frame.data_mut(i).as_mut_ptr();
                    image.as_mut().yuvRowBytes[i] = frame.stride(i) as u32;
                }
                image.as_mut().alphaPlane = frame.data_mut(3).as_mut_ptr();
                image.as_mut().alphaRowBytes = frame.stride(3) as u32;

                image.as_mut().imageOwnsYUVPlanes = sys::AVIF_FALSE as sys::avifBool;
                image.as_mut().imageOwnsAlphaPlane = sys::AVIF_FALSE as sys::avifBool;
                Some(Self::from_raw(image))
            },
            _ => None,
        }
    }
}

impl Drop for AvifImage {
    fn drop(&mut self) {
        unsafe {
            sys::avifImageDestroy(self.image.as_ptr());
        }
    }
}

impl<'a> AsRef<AvifImage> for BorrowedAvifImage<'a> {
    fn as_ref(&self) -> &AvifImage {
        &self.inner
    }
}

fn create_raw_from_yuv_format(
    frame: &frame::Video,
    format: Pixel,
) -> Option<NonNull<sys::avifImage>> {
    NonNull::new(unsafe {
        sys::avifImageCreate(
            frame.width() as i32,
            frame.height() as i32,
            8,
            match format {
                Pixel::YUV420P => sys::AVIF_PIXEL_FORMAT_YUV420,
                Pixel::YUV422P => sys::AVIF_PIXEL_FORMAT_YUV422,
                Pixel::YUV444P => sys::AVIF_PIXEL_FORMAT_YUV444,
                _ => return None,
            },
        )
    })
}
