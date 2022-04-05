//! Mostly the same as libavif-rs::AvifData, with the only difference that we don't create it
//! so we know it's from libavif
use libavif_sys as sys;

pub struct AvifRwData {
    data: sys::avifRWData,
}

impl AvifRwData {
    /// Safety: `data` must be a valid value obtained from libavif
    /// which must have not been freed yet.
    pub(crate) unsafe fn from_raw(data: sys::avifRWData) -> Self {
        Self { data }
    }

    /// Extracts a slice containg the entire data without doing clones or allocation.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.data, self.data.size) }
    }

    pub fn len(&self) -> usize {
        self.data.size
    }
}

impl std::ops::Deref for AvifRwData {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl Drop for AvifRwData {
    fn drop(&mut self) {
        unsafe {
            sys::avifRWDataFree(&mut self.data);
        }
    }
}

unsafe impl Send for AvifRwData {}
