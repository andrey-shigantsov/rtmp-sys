pub use libc::*;

pub type va_list = __builtin_va_list;

type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}
impl ::std::default::Default for __va_list_tag {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn inet_addr(__cp: *const c_char) -> in_addr_t;
    pub fn inet_lnaof(__in: in_addr) -> in_addr_t;
    pub fn inet_makeaddr(__net: in_addr_t, __host: in_addr_t) -> in_addr;
    pub fn inet_netof(__in: in_addr) -> in_addr_t;
    pub fn inet_network(__cp: *const c_char) -> in_addr_t;
    pub fn inet_ntoa(__in: in_addr) -> *mut c_char;
    pub fn inet_pton(__af: c_int,
                     __cp: *const c_char,
                     __buf: *mut c_void)
     -> c_int;
    pub fn inet_ntop(__af: c_int,
                     __cp: *const c_void,
                     __buf: *mut c_char, __len: socklen_t)
     -> *const c_char;
    pub fn inet_aton(__cp: *const c_char, __inp: *mut in_addr)
     -> c_int;
    pub fn inet_neta(__net: in_addr_t, __buf: *mut c_char,
                     __len: size_t) -> *mut c_char;
    pub fn inet_net_ntop(__af: c_int,
                         __cp: *const c_void,
                         __bits: c_int,
                         __buf: *mut c_char, __len: size_t)
     -> *mut c_char;
    pub fn inet_net_pton(__af: c_int,
                         __cp: *const c_char,
                         __buf: *mut c_void, __len: size_t)
     -> c_int;
    pub fn inet_nsap_addr(__cp: *const c_char,
                          __buf: *mut c_uchar,
                          __len: c_int)
     -> c_uint;
    pub fn inet_nsap_ntoa(__len: c_int,
                          __cp: *const c_uchar,
                          __buf: *mut c_char)
     -> *mut c_char;

     pub fn ntohl(__netlong: u32) -> u32;
     pub fn ntohs(__netshort: u16) -> u16;
     pub fn htonl(__hostlong: u32) -> u32;
     pub fn htons(__hostshort: u16) -> u16; 
}