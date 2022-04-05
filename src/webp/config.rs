use super::types::{AlphaFiltering, ImageHint, PreprocessingFilter};
use crate::{apply_options, cli, getter_debug, getter_setter, webp::Preset};
use libwebp_sys as sys;
use std::{
    fmt::{Debug, Formatter},
    os::raw::c_int,
};

pub struct Config {
    conf: sys::WebPConfig,
}

impl Config {
    pub fn new(preset: Preset) -> Option<Self> {
        unsafe {
            let mut conf = std::mem::MaybeUninit::uninit();
            if sys::WebPConfigInitInternal(
                conf.as_mut_ptr(),
                preset.into(),
                75.5,
                sys::WEBP_DECODER_ABI_VERSION,
            ) != 0
            {
                Some(Self {
                    conf: conf.assume_init(),
                })
            } else {
                None
            }
        }
    }

    getter_setter!(lossless!);
    getter_setter!(quality, f32: clamp 0.0 to 100.0);
    getter_setter!(method, c_int);
    getter_setter!(image_hint, into ImageHint);
    getter_setter!(target_size, c_int);
    getter_setter!(target_PSNR as target_psnr, f32);
    getter_setter!(segments, c_int: clamp 1 to 4);
    getter_setter!(sns_strength, c_int: clamp 0 to 100);
    getter_setter!(filter_strength, c_int: clamp 0 to 100);
    getter_setter!(filter_sharpness, c_int: clamp 0 to 7);
    getter_setter!(filter_type as strong_filter!);
    getter_setter!(autofilter!);
    getter_setter!(alpha_compression!);
    getter_setter!(alpha_filtering, into AlphaFiltering);
    getter_setter!(alpha_quality, c_int: clamp 0 to 100);
    getter_setter!(pass, c_int: clamp 0 to 100);
    getter_setter!(show_compressed!);
    getter_setter!(preprocessing, into PreprocessingFilter);
    getter_setter!(partitions, c_int: clamp 0 to 3);
    getter_setter!(partition_limit, c_int: clamp 0 to 100);
    getter_setter!(emulate_jpeg_size!);
    getter_setter!(thread_level!);
    getter_setter!(low_memory!);
    getter_setter!(near_lossless, c_int: clamp 0 to 100);
    getter_setter!(exact!);
    getter_setter!(use_delta_palette!);
    getter_setter!(use_sharp_yuv!);
    // not yet used
    //getter_setter!(qmin, c_int);
    //getter_setter!(qmax, c_int);

    pub fn validate(&self) -> bool {
        unsafe { sys::WebPValidateConfig(&self.conf) != 0 }
    }

    pub fn apply_cli_options(&mut self, opts: &cli::WebpOptions) {
        apply_options!(self, opts;
            lossless,
            quality,
            method,
            image_hint,
            target_size,
            target_psnr,
            segments,
            sns_strength,
            filter_strength,
            filter_sharpness,
            strong_filter,
            autofilter,
            alpha_compression,
            alpha_filtering,
            alpha_quality,
            pass,
            show_compressed,
            preprocessing,
            partitions,
            partition_limit,
            emulate_jpeg_size,
            thread_level,
            low_memory,
            near_lossless,
            exact,
            use_delta_palette,
            use_sharp_yuv,
        );
    }

    pub(super) unsafe fn as_ptr(&self) -> *const sys::WebPConfig {
        &self.conf
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        getter_debug!(
            self, f, "Config";
            lossless,
            quality,
            method,
            image_hint,
            target_size,
            target_psnr,
            segments,
            sns_strength,
            filter_strength,
            filter_sharpness,
            strong_filter,
            autofilter,
            alpha_compression,
            alpha_filtering,
            alpha_quality,
            pass,
            show_compressed,
            preprocessing,
            partitions,
            partition_limit,
            emulate_jpeg_size,
            thread_level,
            low_memory,
            near_lossless,
            exact,
            use_delta_palette,
            use_sharp_yuv,
        )
    }
}
