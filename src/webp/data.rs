use libwebp_sys as sys;

pub struct WebpData {
    data: sys::WebPData,
}

impl WebpData {
    pub(super) fn new() -> Self {
        let mut data = sys::WebPData::default();
        sys::WebPDataInit(&mut data);
        Self { data }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data.bytes, self.data.size) }
    }

    pub fn len(&self) -> usize {
        self.data.size
    }

    pub(super) unsafe fn as_mut_ptr(&mut self) -> *mut sys::WebPData {
        &mut self.data
    }
}

impl AsRef<[u8]> for WebpData {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Drop for WebpData {
    fn drop(&mut self) {
        unsafe {
            sys::WebPDataClear(&mut self.data);
        }
    }
}
