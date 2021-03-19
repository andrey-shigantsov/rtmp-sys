#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case,
         improper_ctypes)]

use crate::common::*;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum RTMP_LogLevel {
    RTMP_LOGCRIT = 0,
    RTMP_LOGERROR = 1,
    RTMP_LOGWARNING = 2,
    RTMP_LOGINFO = 3,
    RTMP_LOGDEBUG = 4,
    RTMP_LOGDEBUG2 = 5,
    RTMP_LOGALL = 6,
}
pub type RTMP_LogCallback_raw = unsafe extern "C" fn (
    level: ::std::os::raw::c_int,
    fmt: *const c_char,
    arg1: va_list
);
pub type RTMP_LogCallback = Option<RTMP_LogCallback_raw>;

extern "C" {
    pub static mut RTMP_debuglevel: RTMP_LogLevel;
    pub fn RTMP_LogSetCallback(cb: RTMP_LogCallback);
    pub fn RTMP_LogSetOutput(file: *mut FILE);
    pub fn RTMP_LogPrintf(format: *const ::std::os::raw::c_char, ...);
    pub fn RTMP_LogStatus(format: *const ::std::os::raw::c_char, ...);
    pub fn RTMP_Log(level: ::std::os::raw::c_int,
                    format: *const ::std::os::raw::c_char, ...);
    pub fn RTMP_LogHex(level: ::std::os::raw::c_int, data: *const u8,
                       len: ::std::os::raw::c_ulong);
    pub fn RTMP_LogHexString(level: ::std::os::raw::c_int,
                             data: *const u8,
                             len: ::std::os::raw::c_ulong);
    pub fn RTMP_LogSetLevel(lvl: RTMP_LogLevel);
    pub fn RTMP_LogGetLevel() -> RTMP_LogLevel;
}
