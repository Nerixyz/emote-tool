use libavif_sys as sys;

pub enum AddImage {
    None = sys::AVIF_ADD_IMAGE_FLAG_NONE as isize,
    ForceKeyframe = sys::AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME as isize,
    Single = sys::AVIF_ADD_IMAGE_FLAG_SINGLE as isize,
}

impl From<AddImage> for sys::avifAddImageFlags {
    fn from(flags: AddImage) -> Self {
        flags as Self
    }
}
