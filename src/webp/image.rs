use crate::webp::errors;
use ffmpeg_next::{format, frame};
use libwebp_sys as sys;
use std::{marker::PhantomData, mem::ManuallyDrop, ptr::NonNull};

// we don't need to implement drop for WebpImage since it contains a reference to the video data
// and nothing from libwebp is allocated (WebPPictureInitInternal only zeroes the memory)

pub struct WebpImage<'a> {
    picture: sys::WebPPicture,
    _marker: PhantomData<&'a frame::Video>,
}

impl<'a> WebpImage<'a> {
    fn new() -> Result<Self, errors::CreateImageError> {
        unsafe {
            let mut picture = std::mem::MaybeUninit::uninit();
            if sys::WebPPictureInit(picture.as_mut_ptr()) {
                Ok(Self {
                    picture: picture.assume_init(),
                    _marker: PhantomData::default(),
                })
            } else {
                Err(errors::CreateImageError::CannotCreateImage)
            }
        }
    }

    pub fn from_av_frame(frame: &'a mut frame::Video) -> Result<Self, errors::CreateImageError> {
        let is_alpha = match frame.format() {
            format::Pixel::YUV420P => false,
            format::Pixel::YUVA420P => true,
            _ => return Err(errors::CreateImageError::InvalidPixelFormat),
        };

        let mut pic = Self::new()?;
        pic.picture.colorspace = if is_alpha {
            sys::WebPEncCSP::WEBP_YUV420A
        } else {
            sys::WebPEncCSP::WEBP_YUV420
        };
        let planes = frame.planes();
        assert_eq!(planes, if is_alpha { 4 } else { 3 });
        assert_eq!(frame.stride(1), frame.stride(2));
        pic.picture.y = frame.data_mut(0).as_mut_ptr();
        pic.picture.y_stride = frame.stride(0) as _;
        pic.picture.u = frame.data_mut(1).as_mut_ptr();
        pic.picture.v = frame.data_mut(2).as_mut_ptr();
        pic.picture.uv_stride = frame.stride(1) as _;
        if is_alpha {
            pic.picture.a = frame.data_mut(3).as_mut_ptr();
            pic.picture.a_stride = frame.stride(3) as _;
        }
        pic.picture.width = frame.width() as _;
        pic.picture.height = frame.height() as _;

        Ok(pic)
    }

    pub fn set_writer(&mut self, writer: sys::WebPWriterFunction) {
        self.picture.writer = writer;
    }

    pub fn error_code(&self) -> sys::WebPEncodingError {
        self.picture.error_code
    }

    pub(super) fn set_user_data(&mut self, any: Box<dyn std::any::Any>) {
        self.picture.user_data = Box::into_raw(any) as *mut _;
    }

    pub fn take_user_data<T: 'static>(&mut self) -> Option<Box<T>> {
        if !self.picture.user_data.is_null() {
            let any_box =
                unsafe { Box::from_raw(self.picture.user_data as *mut dyn std::any::Any) };
            self.picture.user_data = std::ptr::null_mut();
            match any_box.downcast() {
                // box is moved here
                Ok(boxed) => Some(boxed),
                // box is dropped here
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn clone_stats(&self) -> Option<sys::WebPAuxStats> {
        unsafe { NonNull::new(self.picture.stats).map(|p| p.as_ref().clone()) }
    }

    pub(super) unsafe fn as_mut_ptr(&mut self) -> *mut sys::WebPPicture {
        &mut self.picture
    }
}

pub(super) fn with_user_data<T, F>(pic: *const sys::WebPPicture, func: F) -> bool
where
    T: 'static,
    F: FnOnce(&sys::WebPPicture, &mut T) -> bool,
{
    unsafe {
        if pic.is_null() || (*pic).user_data.is_null() {
            return false;
        }
        let any_box = Box::from_raw((*pic).user_data as *mut dyn std::any::Any);
        match any_box.downcast::<T>() {
            // box is moved here
            Ok(boxed) => {
                let mut boxed = ManuallyDrop::new(boxed);
                func(&*pic, &mut boxed)
                // boxed is not dropped here
            }
            Err(_) => false,
        }
    }
}

impl Drop for WebpImage<'_> {
    fn drop(&mut self) {
        if !self.picture.user_data.is_null() {
            eprintln!(
                "[WebpImage] User data was not null when dropping - indicating it wasn't used."
            );
            unsafe { drop(Box::from_raw(self.picture.user_data)) }
        }
    }
}
