use libavif_sys as sys;

pub enum AddImageFlags {
    None = sys::AVIF_ADD_IMAGE_FLAG_NONE as isize,
    ForceKeyframe = sys::AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as isize,
    Single = sys::AVIF_ADD_IMAGE_FLAG_SINGLE as isize,
}

impl From<AddImageFlags> for sys::avifAddImageFlags {
    fn from(flags: AddImageFlags) -> Self {
        flags as Self
    }
}
