use crate::common::*;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum HTTPResult {
    HTTPRES_OK = 0,
    HTTPRES_OK_NOT_MODIFIED = 1,
    HTTPRES_NOT_FOUND = 2,
    HTTPRES_BAD_REQUEST = 3,
    HTTPRES_SERVER_ERROR = 4,
    HTTPRES_REDIRECTED = 5,
    HTTPRES_LOST_CONNECTION = 6,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct HTTP_ctx {
    pub date: *mut c_char,
    pub size: c_int,
    pub status: c_int,
    pub data: *mut c_void,
}
impl ::std::default::Default for HTTP_ctx {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type HTTP_read_callback_raw = unsafe extern "C" fn (
        ptr: *mut c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut c_void
    ) -> c_ulong;

pub type HTTP_read_callback = Option<HTTP_read_callback_raw>;
extern "C" {
    pub fn HTTP_get (
        http: *mut HTTP_ctx,
        url: *const c_char,
        cb: HTTP_read_callback
    ) -> HTTPResult;
}
