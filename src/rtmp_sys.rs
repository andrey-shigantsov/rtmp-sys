pub use crate::common::*;

use std::io::Error as ioError;

pub use crate::rtmp::*;

#[inline]
fn GetSockError() -> ioError {
    ioError::last_os_error()
}
#[inline]
fn SetSockError<E>(_e: E) {
    unimplemented!()
}
#[inline]
unsafe fn closesocket(s: c_int) -> c_int { close(s) }
#[inline]
unsafe fn msleep(n: c_uint) -> c_int { usleep(n*1000) }

// FIXME
// #[inline]
// fn SET_RCVTIMEO(tv,s) { timeval tv = {s,0} }
