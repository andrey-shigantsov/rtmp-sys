#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use crate::common::*;

pub use crate::amf::*;

pub const RTMP_LIB_VERSION: i32 = 0x020300;	/* 2.3 */

pub const RTMP_FEATURE_HTTP: i32 = 0x01;
pub const RTMP_FEATURE_ENC: i32 = 0x02;
pub const RTMP_FEATURE_SSL: i32 = 0x04;
pub const RTMP_FEATURE_MFP: i32 = 0x08;	/* not yet supported */
pub const RTMP_FEATURE_WRITE: i32 = 0x10;	/* publish, not play */
pub const RTMP_FEATURE_HTTP2: i32 = 0x20;	/* server-side rtmpt */

pub const RTMP_PROTOCOL_UNDEFINED: i32 = -1;
pub const RTMP_PROTOCOL_RTMP: i32 = 0;
pub const RTMP_PROTOCOL_RTMPE: i32 = RTMP_FEATURE_ENC;
pub const RTMP_PROTOCOL_RTMPT: i32 = RTMP_FEATURE_HTTP;
pub const RTMP_PROTOCOL_RTMPS: i32 = RTMP_FEATURE_SSL;
pub const RTMP_PROTOCOL_RTMPTE: i32 = RTMP_FEATURE_HTTP|RTMP_FEATURE_ENC;
pub const RTMP_PROTOCOL_RTMPTS: i32 = RTMP_FEATURE_HTTP|RTMP_FEATURE_SSL;
pub const RTMP_PROTOCOL_RTMFP: i32 = RTMP_FEATURE_MFP;

pub const RTMP_DEFAULT_CHUNKSIZE: i32 = 128;

/* needs to fit largest number of bytes recv() may return */
pub const RTMP_BUFFER_CACHE_SIZE: i32 = 16*1024;

pub const RTMP_CHANNELS: i32 = 65600;

/*      RTMP_PACKET_TYPE_...                0x00 */
pub const RTMP_PACKET_TYPE_CHUNK_SIZE: i32 = 0x01;
/*      RTMP_PACKET_TYPE_...                0x02 */
pub const RTMP_PACKET_TYPE_BYTES_READ_REPORT: i32 = 0x03;
pub const RTMP_PACKET_TYPE_CONTROL: i32 = 0x04;
pub const RTMP_PACKET_TYPE_SERVER_BW: i32 = 0x05;
pub const RTMP_PACKET_TYPE_CLIENT_BW: i32 = 0x06;
/*      RTMP_PACKET_TYPE_...                0x07 */
pub const RTMP_PACKET_TYPE_AUDIO: i32 = 0x08;
pub const RTMP_PACKET_TYPE_VIDEO: i32 = 0x09;
/*      RTMP_PACKET_TYPE_...                0x0A */
/*      RTMP_PACKET_TYPE_...                0x0B */
/*      RTMP_PACKET_TYPE_...                0x0C */
/*      RTMP_PACKET_TYPE_...                0x0D */
/*      RTMP_PACKET_TYPE_...                0x0E */
pub const RTMP_PACKET_TYPE_FLEX_STREAM_SEND: i32 = 0x0F;
pub const RTMP_PACKET_TYPE_FLEX_SHARED_OBJECT: i32 = 0x10;
pub const RTMP_PACKET_TYPE_FLEX_MESSAGE: i32 = 0x11;
pub const RTMP_PACKET_TYPE_INFO: i32 = 0x12;
pub const RTMP_PACKET_TYPE_SHARED_OBJECT: i32 = 0x13;
pub const RTMP_PACKET_TYPE_INVOKE: i32 = 0x14;
/*      RTMP_PACKET_TYPE_...                0x15 */
pub const RTMP_PACKET_TYPE_FLASH_VIDEO: i32 = 0x16;

pub const RTMP_MAX_HEADER_SIZE: i32 = 18;

pub const RTMP_PACKET_SIZE_LARGE: i32 = 0;
pub const RTMP_PACKET_SIZE_MEDIUM: i32 = 1;
pub const RTMP_PACKET_SIZE_SMALL: i32 = 2;
pub const RTMP_PACKET_SIZE_MINIMUM: i32 = 3;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct RTMPChunk {
    pub c_headerSize: c_int,
    pub c_chunkSize: c_int,
    pub c_chunk: *mut c_char,
    pub c_header: [c_char; 18usize],
}
impl ::std::default::Default for RTMPChunk {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct RTMPPacket {
    pub m_headerType: u8,
    pub m_packetType: u8,
    pub m_hasAbsTimestamp: u8,
    pub m_nChannel: c_int,
    pub m_nTimeStamp: u32,
    pub m_nInfoField2: i32,
    pub m_nBodySize: u32,
    pub m_nBytesRead: u32,
    pub m_chunk: *mut RTMPChunk,
    pub m_body: *mut c_char,
}
impl ::std::default::Default for RTMPPacket {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct RTMPSockBuf {
    pub sb_socket: c_int,
    pub sb_size: c_int,
    pub sb_start: *mut c_char,
    pub sb_buf: [c_char; 16384usize],
    pub sb_timedout: c_int,
    pub sb_ssl: *mut c_void,
}
impl ::std::clone::Clone for RTMPSockBuf {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for RTMPSockBuf {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct RTMP_LNK {
    pub hostname: AVal,
    pub sockshost: AVal,
    pub playpath0: AVal,
    pub playpath: AVal,
    pub tcUrl: AVal,
    pub swfUrl: AVal,
    pub pageUrl: AVal,
    pub app: AVal,
    pub auth: AVal,
    pub flashVer: AVal,
    pub subscribepath: AVal,
    pub usherToken: AVal,
    pub token: AVal,
    pub pubUser: AVal,
    pub pubPasswd: AVal,
    pub extras: AMFObject,
    pub edepth: c_int,
    pub seekTime: c_int,
    pub stopTime: c_int,
    pub lFlags: c_int,
    pub swfAge: c_int,
    pub protocol: c_int,
    pub timeout: c_int,
    pub pFlags: c_int,
    pub socksport: c_ushort,
    pub port: c_ushort,
    pub dh: *mut c_void,
    pub rc4keyIn: *mut c_void,
    pub rc4keyOut: *mut c_void,
    pub SWFSize: u32,
    pub SWFHash: [u8; 32usize],
    pub SWFVerificationResponse: [c_char; 42usize],
}
impl ::std::clone::Clone for RTMP_LNK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for RTMP_LNK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct RTMP_READ {
    pub buf: *mut c_char,
    pub bufpos: *mut c_char,
    pub buflen: c_uint,
    pub timestamp: u32,
    pub dataType: u8,
    pub flags: u8,
    pub status: i8,
    pub initialFrameType: u8,
    pub nResumeTS: u32,
    pub metaHeader: *mut c_char,
    pub initialFrame: *mut c_char,
    pub nMetaHeaderSize: u32,
    pub nInitialFrameSize: u32,
    pub nIgnoredFrameCounter: u32,
    pub nIgnoredFlvFrameCounter: u32,
}
impl ::std::default::Default for RTMP_READ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct RTMP_METHOD {
    pub name: AVal,
    pub num: c_int,
    _bindgen_padding_0_: [u8; 4usize],
}
impl ::std::default::Default for RTMP_METHOD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
// #[derive(Debug)]
pub struct RTMP {
    pub m_inChunkSize: c_int,
    pub m_outChunkSize: c_int,
    pub m_nBWCheckCounter: c_int,
    pub m_nBytesIn: c_int,
    pub m_nBytesInSent: c_int,
    pub m_nBufferMS: c_int,
    pub m_stream_id: c_int,
    pub m_mediaChannel: c_int,
    pub m_mediaStamp: u32,
    pub m_pauseStamp: u32,
    pub m_pausing: c_int,
    pub m_nServerBW: c_int,
    pub m_nClientBW: c_int,
    pub m_nClientBW2: u8,
    pub m_bPlaying: u8,
    pub m_bSendEncoding: u8,
    pub m_bSendCounter: u8,
    pub m_numInvokes: c_int,
    pub m_numCalls: c_int,
    pub m_methodCalls: *mut RTMP_METHOD,
    pub m_channelsAllocatedIn: c_int,
    pub m_channelsAllocatedOut: c_int,
    pub m_vecChannelsIn: *mut *mut RTMPPacket,
    pub m_vecChannelsOut: *mut *mut RTMPPacket,
    pub m_channelTimestamp: *mut c_int,
    pub m_fAudioCodecs: f64,
    pub m_fVideoCodecs: f64,
    pub m_fEncoding: f64,
    pub m_fDuration: f64,
    pub m_msgCounter: c_int,
    pub m_polling: c_int,
    pub m_resplen: c_int,
    pub m_unackd: c_int,
    pub m_clientID: AVal,
    pub m_read: RTMP_READ,
    pub m_write: RTMPPacket,
    pub m_sb: RTMPSockBuf,
    pub Link: RTMP_LNK,
}
impl ::std::default::Default for RTMP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

extern "C" {
    pub static mut RTMPProtocolStringsLower: [[c_char; 7usize]; 0usize];
    pub static RTMP_DefaultFlashVer: AVal;
    pub static mut RTMP_ctrlC: c_int;
}
extern "C" {
    pub fn RTMP_GetTime() -> u32;
    pub fn RTMPPacket_Reset(p: *mut RTMPPacket);
    pub fn RTMPPacket_Dump(p: *mut RTMPPacket);
    pub fn RTMPPacket_Alloc(p: *mut RTMPPacket, nSize: u32) -> c_int;
    pub fn RTMPPacket_Free(p: *mut RTMPPacket);
    pub fn RTMP_ParseURL(url: *const c_char,
                         protocol: *mut c_int,
                         host: *mut AVal,
                         port: *mut c_uint,
                         playpath: *mut AVal,
                         app: *mut AVal)
                         -> c_int;
    pub fn RTMP_ParsePlaypath(in_: *mut AVal, out: *mut AVal);
    pub fn RTMP_SetBufferMS(r: *mut RTMP, size: c_int);
    pub fn RTMP_UpdateBufferMS(r: *mut RTMP);
    pub fn RTMP_SetOpt(r: *mut RTMP, opt: *const AVal, arg: *mut AVal) -> c_int;
    pub fn RTMP_SetupURL(r: *mut RTMP, url: *mut c_char) -> c_int;
    pub fn RTMP_SetupStream(r: *mut RTMP,
                            protocol: c_int,
                            hostname: *mut AVal,
                            port: c_uint,
                            sockshost: *mut AVal,
                            playpath: *mut AVal,
                            tcUrl: *mut AVal,
                            swfUrl: *mut AVal,
                            pageUrl: *mut AVal,
                            app: *mut AVal,
                            auth: *mut AVal,
                            swfSHA256Hash: *mut AVal,
                            swfSize: u32,
                            flashVer: *mut AVal,
                            subscribepath: *mut AVal,
                            usherToken: *mut AVal,
                            dStart: c_int,
                            dStop: c_int,
                            bLiveStream: c_int,
                            timeout: c_long);
    pub fn RTMP_Connect(r: *mut RTMP, cp: *mut RTMPPacket) -> c_int;
    pub fn RTMP_Connect0(r: *mut RTMP, svc: *mut sockaddr) -> c_int;
    pub fn RTMP_Connect1(r: *mut RTMP, cp: *mut RTMPPacket) -> c_int;
    pub fn RTMP_Serve(r: *mut RTMP) -> c_int;
    pub fn RTMP_TLS_Accept(r: *mut RTMP,
                           ctx: *mut c_void)
                           -> c_int;
    pub fn RTMP_ReadPacket(r: *mut RTMP, packet: *mut RTMPPacket) -> c_int;
    pub fn RTMP_SendPacket(r: *mut RTMP,
                           packet: *mut RTMPPacket,
                           queue: c_int)
                           -> c_int;
    pub fn RTMP_SendChunk(r: *mut RTMP, chunk: *mut RTMPChunk) -> c_int;
    pub fn RTMP_IsConnected(r: *mut RTMP) -> c_int;
    pub fn RTMP_Socket(r: *mut RTMP) -> c_int;
    pub fn RTMP_IsTimedout(r: *mut RTMP) -> c_int;
    pub fn RTMP_GetDuration(r: *mut RTMP) -> f64;
    pub fn RTMP_ToggleStream(r: *mut RTMP) -> c_int;
    pub fn RTMP_ConnectStream(r: *mut RTMP,
                              seekTime: c_int)
                              -> c_int;
    pub fn RTMP_ReconnectStream(r: *mut RTMP,
                                seekTime: c_int)
                                -> c_int;
    pub fn RTMP_DeleteStream(r: *mut RTMP);
    pub fn RTMP_GetNextMediaPacket(r: *mut RTMP, packet: *mut RTMPPacket) -> c_int;
    pub fn RTMP_ClientPacket(r: *mut RTMP, packet: *mut RTMPPacket) -> c_int;
    pub fn RTMP_Init(r: *mut RTMP);
    pub fn RTMP_Close(r: *mut RTMP);
    pub fn RTMP_Alloc() -> *mut RTMP;
    pub fn RTMP_Free(r: *mut RTMP);
    pub fn RTMP_EnableWrite(r: *mut RTMP);
    pub fn RTMP_TLS_AllocServerContext(cert: *const c_char,
                                       key: *const c_char)
                                       -> *mut c_void;
    pub fn RTMP_TLS_FreeServerContext(ctx: *mut c_void);
    pub fn RTMP_LibVersion() -> c_int;
    pub fn RTMP_UserInterrupt();
    pub fn RTMP_SendCtrl(r: *mut RTMP,
                         nType: c_short,
                         nObject: c_uint,
                         nTime: c_uint)
                         -> c_int;
    pub fn RTMP_SendPause(r: *mut RTMP,
                          DoPause: c_int,
                          dTime: c_int)
                          -> c_int;
    pub fn RTMP_Pause(r: *mut RTMP, DoPause: c_int) -> c_int;
    pub fn RTMP_FindFirstMatchingProperty(obj: *mut AMFObject,
                                          name: *const AVal,
                                          p: *mut AMFObjectProperty)
                                          -> c_int;
    pub fn RTMPSockBuf_Fill(sb: *mut RTMPSockBuf) -> c_int;
    pub fn RTMPSockBuf_Send(sb: *mut RTMPSockBuf,
                            buf: *const c_char,
                            len: c_int)
                            -> c_int;
    pub fn RTMPSockBuf_Close(sb: *mut RTMPSockBuf) -> c_int;
    pub fn RTMP_SendCreateStream(r: *mut RTMP) -> c_int;
    pub fn RTMP_SendSeek(r: *mut RTMP, dTime: c_int) -> c_int;
    pub fn RTMP_SendServerBW(r: *mut RTMP) -> c_int;
    pub fn RTMP_SendClientBW(r: *mut RTMP) -> c_int;
    pub fn RTMP_DropRequest(r: *mut RTMP,
                            i: c_int,
                            freeit: c_int);
    pub fn RTMP_Read(r: *mut RTMP,
                     buf: *mut c_char,
                     size: c_int)
                     -> c_int;
    pub fn RTMP_Write(r: *mut RTMP,
                      buf: *const c_char,
                      size: c_int)
                      -> c_int;
    pub fn RTMP_HashSWF(url: *const c_char,
                        size: *mut c_uint,
                        hash: *mut c_uchar,
                        age: c_int)
                        -> c_int;
    pub fn create_server(port: c_int) -> c_int;
}
