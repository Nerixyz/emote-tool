use crate::webp::{image::WebpImage, image_encode};
use libwebp_sys as sys;
use std::{ffi::CStr, io, ptr::NonNull};

#[derive(Debug, thiserror::Error)]
pub enum AnimEncoderError {
    #[error("libwebp indicated an error but we can't convert it into a Rust String")]
    InvalidErrorString,
    #[error("Encoder Failed: {0:?}")]
    EncoderError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateImageError {
    #[error("Frame had an invalid pixel format")]
    InvalidPixelFormat,
    #[error("Couldn't create a webp picture")]
    CannotCreateImage,
}

#[derive(Debug, thiserror::Error)]
pub enum StillEncoderError {
    #[error("Encoding Error: {0:?}")]
    EncodingError(sys::WebPEncodingError),
    #[error("IO Error: {0}")]
    IoError(io::Error),
    #[error("Expected user-data but it was either missing or corrupt")]
    CorruptedUserData,
}

impl AnimEncoderError {
    pub(super) unsafe fn last_error(encoder: &NonNull<sys::WebPAnimEncoder>) -> Self {
        let error = sys::WebPAnimEncoderGetError(encoder.as_ptr());
        match CStr::from_ptr(error).to_str() {
            Ok(str) => Self::EncoderError(str.to_string()),
            Err(_) => Self::InvalidErrorString,
        }
    }
}

impl StillEncoderError {
    pub(super) unsafe fn from_image(image: &mut WebpImage) -> Self {
        match image.take_user_data::<image_encode::UserData>() {
            Some(user_data) => match *user_data {
                Ok(_) => Self::EncodingError(image.error_code()),
                Err(e) => Self::IoError(e),
            },
            None => Self::CorruptedUserData,
        }
    }
}
