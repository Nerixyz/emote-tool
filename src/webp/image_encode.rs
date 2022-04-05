use crate::webp::{
    config::Config,
    errors,
    image::{with_user_data, WebpImage},
};
use libwebp_sys as sys;
use libwebp_sys::WebPPicture;
use std::{io, io::Write, os::raw::c_int};

pub struct WriteData {
    writer: Box<dyn io::Write>,
}

pub type UserData = Result<WriteData, io::Error>;

extern "C" fn writer_write(data: *const u8, len: usize, pic: *const WebPPicture) -> c_int {
    if with_user_data::<UserData, _>(pic, move |_pic, user_data| match user_data {
        Ok(WriteData { writer }) => unsafe {
            match writer.write_all(std::slice::from_raw_parts(data, len)) {
                Ok(_) => true,
                Err(e) => {
                    *user_data = Err(e);
                    false
                }
            }
        },
        Err(_) => false,
    }) {
        1
    } else {
        0
    }
}

pub fn encode_image<W: io::Write + 'static>(
    image: &mut WebpImage,
    writer: W,
    config: &Config,
) -> Result<Option<sys::WebPAuxStats>, errors::StillEncoderError> {
    unsafe {
        image.set_user_data(Box::<UserData>::new(Ok(WriteData {
            writer: Box::new(writer),
        })));
        image.set_writer(Some(writer_write));
        if sys::WebPEncode(config.as_ptr(), image.as_mut_ptr()) == 0 {
            Err(errors::StillEncoderError::from_image(image))
        } else {
            Ok(image.clone_stats())
        }
    }
}
