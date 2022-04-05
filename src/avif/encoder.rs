use crate::avif::{data::AvifRwData, flags::AddImageFlags, image::AvifImage, Codec};
use libavif_sys as sys;
use libavif_sys::avifIOStats;
use std::{
    fmt::{Debug, Formatter},
    ptr::NonNull,
};

pub struct Encoder {
    encoder: NonNull<sys::avifEncoder>,
}

impl Encoder {
    pub fn new() -> Option<Self> {
        Some(Self {
            encoder: NonNull::new(unsafe { sys::avifEncoderCreate() })?,
        })
    }

    /// Get the maximum allowed number of threads this `Encoder` can use
    pub fn max_threads(&self) -> usize {
        unsafe { self.encoder.as_ref().maxThreads as usize }
    }

    /// Set the maximum allowed number of threads this `Encoder` can use
    pub fn set_max_threads(&mut self, max_threads: usize) -> &mut Self {
        unsafe { self.encoder.as_mut().maxThreads = max_threads.max(1) as i32 }
        self
    }

    /// Get quantizer value for the YUV channels
    pub fn quantizer(&self) -> u8 {
        unsafe { self.encoder.as_ref().minQuantizer as u8 }
    }

    /// Set the quantizer value for the YUV channels
    ///
    /// Must be between 0 and 63.
    ///
    /// * `0` - _lossless_
    /// * `63` - _lowest quality_
    pub fn set_quantizer(&mut self, quantizer: u8) -> &mut Self {
        let quantizer = quantizer.min(63) as i32;
        unsafe {
            self.encoder.as_mut().minQuantizer = quantizer;
            self.encoder.as_mut().maxQuantizer = quantizer;
        }
        self
    }

    /// Get quantizer value for the alpha channel
    pub fn quantizer_alpha(&self) -> u8 {
        unsafe { self.encoder.as_ref().minQuantizerAlpha as u8 }
    }

    /// Set the quantizer value for the alpha channel
    ///
    /// Must be between 0 and 63.
    ///
    /// * `0` - _lossless_
    /// * `63` - _lowest quality_
    pub fn set_quantizer_alpha(&mut self, quantizer_alpha: u8) -> &mut Self {
        let quantizer_alpha = quantizer_alpha.min(63) as i32;
        unsafe {
            self.encoder.as_mut().minQuantizerAlpha = quantizer_alpha;
            self.encoder.as_mut().maxQuantizerAlpha = quantizer_alpha;
        }
        self
    }

    /// Get the speed of this `Encoder`
    pub fn speed(&self) -> u8 {
        unsafe { self.encoder.as_ref().speed as u8 }
    }

    /// Set the speed of this `Encoder`
    ///
    /// Must be between 0 and 10.
    ///
    /// * `10` - _fastest_
    /// * `0` - _slowest_
    pub fn set_speed(&mut self, speed: u8) -> &mut Self {
        unsafe { self.encoder.as_mut().speed = speed.min(10) as i32 }
        self
    }

    pub fn timescale(&self) -> u64 {
        unsafe { self.encoder.as_ref().timescale }
    }

    pub fn set_timescale(&mut self, timescale: u64) -> &mut Self {
        unsafe { self.encoder.as_mut().timescale = timescale }
        self
    }

    pub fn codec(&self) -> Codec {
        unsafe { self.encoder.as_ref().codecChoice.into() }
    }

    pub fn set_codec(&mut self, codec: Codec) -> &mut Self {
        unsafe { self.encoder.as_mut().codecChoice = codec.into() }
        self
    }

    pub fn stats(&self) -> avifIOStats {
        unsafe { self.encoder.as_ref().ioStats }
    }

    pub fn add_image_none(
        &mut self,
        image: &AvifImage,
        duration_in_timescales: u64,
    ) -> Result<(), super::Error> {
        self.add_image_flags(image, duration_in_timescales, AddImageFlags::None)
    }

    pub fn add_image_flags(
        &mut self,
        image: &AvifImage,
        duration_in_timescales: u64,
        flags: AddImageFlags,
    ) -> Result<(), super::Error> {
        unsafe {
            super::Error::from_code(sys::avifEncoderAddImage(
                self.encoder.as_ptr(),
                image.inner(),
                duration_in_timescales,
                flags.into(),
            ))
        }
    }

    pub fn finish(&mut self) -> Result<AvifRwData, super::Error> {
        unsafe {
            let mut data = sys::avifRWData::default();
            super::Error::from_code(sys::avifEncoderFinish(self.encoder.as_ptr(), &mut data))?;
            Ok(AvifRwData::from_raw(data))
        }
    }

    pub fn encode_single_image(&mut self, image: &AvifImage) -> Result<AvifRwData, super::Error> {
        unsafe {
            let mut data = sys::avifRWData::default();
            super::Error::from_code(sys::avifEncoderWrite(
                self.encoder.as_ptr(),
                image.inner(),
                &mut data,
            ))?;
            Ok(AvifRwData::from_raw(data))
        }
    }
}

impl Debug for Encoder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Encoder")
            .field("codec", &self.codec())
            .field("max_threads", &self.max_threads())
            .field("quantizer", &self.quantizer())
            .field("quantizer_alpha", &self.quantizer_alpha())
            .field("speed", &self.speed())
            .field("timescale", &self.timescale())
            .field("stats", &self.stats())
            .finish()
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            sys::avifEncoderDestroy(self.encoder.as_ptr());
        }
    }
}
