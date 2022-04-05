use super::{config::Config, image::WebpImage, options::AnimEncoderOptions};
use crate::webp::{data::WebpData, errors};
use libwebp_sys as sys;
use std::{marker::PhantomData, os::raw::c_int, ptr::NonNull};

pub struct AnimEncoder<'a> {
    encoder: NonNull<sys::WebPAnimEncoder>,
    /// `encoder` keeps a reference to the options
    _opts_marker: PhantomData<&'a AnimEncoderOptions>,
    config: Config,
}

impl<'a> AnimEncoder<'a> {
    pub fn create(
        width: usize,
        height: usize,
        options: &'a AnimEncoderOptions,
        config: Config,
    ) -> Option<Self> {
        let encoder = unsafe {
            sys::WebPAnimEncoderNewInternal(
                width as _,
                height as _,
                options.as_ptr(),
                sys::WEBP_MUX_ABI_VERSION,
            )
        };
        Some(Self {
            _opts_marker: PhantomData::default(),
            encoder: NonNull::new(encoder)?,
            config,
        })
    }

    pub fn add_image(
        &mut self,
        image: &mut WebpImage,
        timestamp_ms: c_int,
    ) -> Result<(), errors::AnimEncoderError> {
        self.add_image_internal(Some(image), timestamp_ms)
    }

    fn add_image_internal(
        &mut self,
        image: Option<&mut WebpImage>,
        timestamp_ms: c_int,
    ) -> Result<(), errors::AnimEncoderError> {
        unsafe {
            if sys::WebPAnimEncoderAdd(
                self.encoder.as_ptr(),
                match image {
                    Some(img) => img.as_mut_ptr(),
                    None => std::ptr::null_mut(),
                },
                timestamp_ms,
                self.config.as_ptr(),
            ) == 0
            {
                Err(errors::AnimEncoderError::last_error(self.encoder))
            } else {
                Ok(())
            }
        }
    }

    pub fn finalize(&mut self, final_ts_ms: c_int) -> Result<WebpData, errors::AnimEncoderError> {
        unsafe {
            self.add_image_internal(None, final_ts_ms)?;
            let mut data = WebpData::new();
            if sys::WebPAnimEncoderAssemble(self.encoder.as_ptr(), data.as_mut_ptr()) == 0 {
                Err(errors::AnimEncoderError::last_error(self.encoder))
            } else {
                Ok(data)
            }
        }
    }
}

impl Drop for AnimEncoder<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::WebPAnimEncoderDelete(self.encoder.as_ptr());
        }
    }
}
