use crate::common::*;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum AMFDataType {
    AMF_NUMBER = 0,
    AMF_BOOLEAN = 1,
    AMF_STRING = 2,
    AMF_OBJECT = 3,
    AMF_MOVIECLIP = 4,
    AMF_NULL = 5,
    AMF_UNDEFINED = 6,
    AMF_REFERENCE = 7,
    AMF_ECMA_ARRAY = 8,
    AMF_OBJECT_END = 9,
    AMF_STRICT_ARRAY = 10,
    AMF_DATE = 11,
    AMF_LONG_STRING = 12,
    AMF_UNSUPPORTED = 13,
    AMF_RECORDSET = 14,
    AMF_XML_DOC = 15,
    AMF_TYPED_OBJECT = 16,
    AMF_AVMPLUS = 17,
    AMF_INVALID = 255,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum AMF3DataType {
    AMF3_UNDEFINED = 0,
    AMF3_NULL = 1,
    AMF3_FALSE = 2,
    AMF3_TRUE = 3,
    AMF3_INTEGER = 4,
    AMF3_DOUBLE = 5,
    AMF3_STRING = 6,
    AMF3_XML_DOC = 7,
    AMF3_DATE = 8,
    AMF3_ARRAY = 9,
    AMF3_OBJECT = 10,
    AMF3_XML = 11,
    AMF3_BYTE_ARRAY = 12,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct AVal {
    pub av_val: *mut c_char,
    pub av_len: c_int,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for AVal {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

// FIXME
// #[inline]
// fn AVC(s: &::std::ffi::CString) -> AVal {
//     AVal { 
//         av_val: s,
//         av_len: libc::sizeof(s)-1,
//         _bindgen_padding_0_: unsafe { ::std::mem::zeroed() },
//     }
// }

unsafe fn AVMATCH(a1: *const AVal, a2: *const AVal) -> bool {
    (*a1).av_len == (*a2).av_len 
    && libc::memcmp(
        (*a1).av_val as *const c_void,
        (*a2).av_val as *const c_void,
        (*a1).av_len as usize) != 0
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct AMFObject {
    pub o_num: c_int,
    pub o_props: *mut AMFObjectProperty,
}
impl ::std::default::Default for AMFObject {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct AMFObjectProperty {
    pub p_name: AVal,
    pub p_type: AMFDataType,
    pub p_vu: Union_Unnamed1,
    pub p_UTCoffset: i16,
    _bindgen_padding_0_: [u8; 6usize],
}
impl ::std::default::Default for AMFObjectProperty {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 2usize],
}
impl Union_Unnamed1 {
    pub unsafe fn p_number(&mut self) -> *mut f64 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn p_aval(&mut self) -> *mut AVal {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn p_object(&mut self) -> *mut AMFObject {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct AMF3ClassDef {
    pub cd_name: AVal,
    pub cd_externalizable: c_char,
    pub cd_dynamic: c_char,
    pub cd_num: c_int,
    pub cd_props: *mut AVal,
}
impl ::std::default::Default for AMF3ClassDef {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn AMF_EncodeString(output: *mut c_char,
                            outend: *mut c_char,
                            str: *const AVal) -> *mut c_char;
    pub fn AMF_EncodeNumber(output: *mut c_char,
                            outend: *mut c_char, dVal: f64)
     -> *mut c_char;
    pub fn AMF_EncodeInt16(output: *mut c_char,
                           outend: *mut c_char,
                           nVal: c_short)
     -> *mut c_char;
    pub fn AMF_EncodeInt24(output: *mut c_char,
                           outend: *mut c_char,
                           nVal: c_int)
     -> *mut c_char;
    pub fn AMF_EncodeInt32(output: *mut c_char,
                           outend: *mut c_char,
                           nVal: c_int)
     -> *mut c_char;
    pub fn AMF_EncodeBoolean(output: *mut c_char,
                             outend: *mut c_char,
                             bVal: c_int)
     -> *mut c_char;
    pub fn AMF_EncodeNamedString(output: *mut c_char,
                                 outend: *mut c_char,
                                 name: *const AVal, value: *const AVal)
     -> *mut c_char;
    pub fn AMF_EncodeNamedNumber(output: *mut c_char,
                                 outend: *mut c_char,
                                 name: *const AVal, dVal: f64)
     -> *mut c_char;
    pub fn AMF_EncodeNamedBoolean(output: *mut c_char,
                                  outend: *mut c_char,
                                  name: *const AVal,
                                  bVal: c_int)
     -> *mut c_char;
    pub fn AMF_DecodeInt16(data: *const c_char)
     -> c_ushort;
    pub fn AMF_DecodeInt24(data: *const c_char)
     -> c_uint;
    pub fn AMF_DecodeInt32(data: *const c_char)
     -> c_uint;
    pub fn AMF_DecodeString(data: *const c_char,
                            str: *mut AVal);
    pub fn AMF_DecodeLongString(data: *const c_char,
                                str: *mut AVal);
    pub fn AMF_DecodeBoolean(data: *const c_char)
     -> c_int;
    pub fn AMF_DecodeNumber(data: *const c_char) -> f64;
    pub fn AMF_Encode(obj: *mut AMFObject,
                      pBuffer: *mut c_char,
                      pBufEnd: *mut c_char)
     -> *mut c_char;
    pub fn AMF_EncodeEcmaArray(obj: *mut AMFObject,
                               pBuffer: *mut c_char,
                               pBufEnd: *mut c_char)
     -> *mut c_char;
    pub fn AMF_EncodeArray(obj: *mut AMFObject,
                           pBuffer: *mut c_char,
                           pBufEnd: *mut c_char)
     -> *mut c_char;
    pub fn AMF_Decode(obj: *mut AMFObject,
                      pBuffer: *const c_char,
                      nSize: c_int,
                      bDecodeName: c_int)
     -> c_int;
    pub fn AMF_DecodeArray(obj: *mut AMFObject,
                           pBuffer: *const c_char,
                           nSize: c_int,
                           nArrayLen: c_int,
                           bDecodeName: c_int)
     -> c_int;
    pub fn AMF3_Decode(obj: *mut AMFObject,
                       pBuffer: *const c_char,
                       nSize: c_int,
                       bDecodeName: c_int)
     -> c_int;
    pub fn AMF_Dump(obj: *mut AMFObject);
    pub fn AMF_Reset(obj: *mut AMFObject);
    pub fn AMF_AddProp(obj: *mut AMFObject, prop: *const AMFObjectProperty);
    pub fn AMF_CountProp(obj: *mut AMFObject) -> c_int;
    pub fn AMF_GetProp(obj: *mut AMFObject, name: *const AVal,
                       nIndex: c_int)
     -> *mut AMFObjectProperty;
    pub fn AMFProp_GetType(prop: *mut AMFObjectProperty) -> AMFDataType;
    pub fn AMFProp_SetNumber(prop: *mut AMFObjectProperty, dval: f64);
    pub fn AMFProp_SetBoolean(prop: *mut AMFObjectProperty,
                              bflag: c_int);
    pub fn AMFProp_SetString(prop: *mut AMFObjectProperty, str: *mut AVal);
    pub fn AMFProp_SetObject(prop: *mut AMFObjectProperty,
                             obj: *mut AMFObject);
    pub fn AMFProp_GetName(prop: *mut AMFObjectProperty, name: *mut AVal);
    pub fn AMFProp_SetName(prop: *mut AMFObjectProperty, name: *mut AVal);
    pub fn AMFProp_GetNumber(prop: *mut AMFObjectProperty) -> f64;
    pub fn AMFProp_GetBoolean(prop: *mut AMFObjectProperty)
     -> c_int;
    pub fn AMFProp_GetString(prop: *mut AMFObjectProperty, str: *mut AVal);
    pub fn AMFProp_GetObject(prop: *mut AMFObjectProperty,
                             obj: *mut AMFObject);
    pub fn AMFProp_IsValid(prop: *mut AMFObjectProperty)
     -> c_int;
    pub fn AMFProp_Encode(prop: *mut AMFObjectProperty,
                          pBuffer: *mut c_char,
                          pBufEnd: *mut c_char)
     -> *mut c_char;
    pub fn AMF3Prop_Decode(prop: *mut AMFObjectProperty,
                           pBuffer: *const c_char,
                           nSize: c_int,
                           bDecodeName: c_int)
     -> c_int;
    pub fn AMFProp_Decode(prop: *mut AMFObjectProperty,
                          pBuffer: *const c_char,
                          nSize: c_int,
                          bDecodeName: c_int)
     -> c_int;
    pub fn AMFProp_Dump(prop: *mut AMFObjectProperty);
    pub fn AMFProp_Reset(prop: *mut AMFObjectProperty);
    pub fn AMF3CD_AddProp(cd: *mut AMF3ClassDef, prop: *mut AVal);
    pub fn AMF3CD_GetProp(cd: *mut AMF3ClassDef, idx: c_int)
     -> *mut AVal;
}
