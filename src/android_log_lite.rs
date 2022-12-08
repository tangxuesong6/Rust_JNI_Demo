// Copyright 2016 The android_log_sys Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::ffi::CStr;
use std::os::raw;

#[allow(non_camel_case_types)]
pub type c_va_list = raw::c_void;
#[allow(non_camel_case_types)]
pub type c_int = raw::c_int;
#[allow(non_camel_case_types)]
pub type c_char = raw::c_char;

// automatically generated by rust-bindgen

#[derive(Clone, Copy)]
#[repr(isize)]
pub enum LogPriority {
    UNKNOWN = 0,
    DEFAULT = 1,
    VERBOSE = 2,
    DEBUG = 3,
    INFO = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    SILENT = 8,
}

// #[allow(non_camel_case_types)]
// #[derive(Clone, Copy)]
// #[non_exhaustive]
// #[repr(i32)]
// pub enum log_id_t {
//     MAIN = 0,
//     RADIO = 1,
//     EVENTS = 2,
//     SYSTEM = 3,
//     CRASH = 4,
//     STATS = 5,
//     SECURITY = 6,
//     KERNEL = 7,
//     MAX = 8,
//     DEFAULT = 0x7FFFFFFF,
// }

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __android_log_message {
    pub struct_size: usize,
    pub buffer_id: i32,
    pub priority: i32,
    pub tag: *const c_char,
    pub file: *const c_char,
    pub line: u32,
    pub message: *const c_char,
}

pub fn v(tag: String, msg: String) {
    rust_android_log(LogPriority::VERBOSE, tag, msg);
}

pub fn d(tag: String, msg: String) {
    rust_android_log(LogPriority::DEBUG, tag, msg);
}

pub fn i(tag: String, msg: String) {
    rust_android_log(LogPriority::INFO, tag, msg);
}

pub fn w(tag: String, msg: String) {
    rust_android_log(LogPriority::WARN, tag, msg);
}

pub fn e(tag: String, msg: String) {
    rust_android_log(LogPriority::ERROR, tag, msg);
}

pub fn rust_android_log(prio: LogPriority, tag: String, msg: String) {
    let android_tag = tag + "\0";
    let android_msg = msg + "\0";
    let tag: &CStr = unsafe { CStr::from_ptr(android_tag.as_ptr().cast()) };
    let msg: &CStr = unsafe { CStr::from_ptr(android_msg.as_ptr().cast()) };
    android_log(prio, tag, msg);
}

pub fn android_log(prio: LogPriority, tag: &CStr, msg: &CStr) {
    unsafe {
        __android_log_write(
            prio as c_int,
            tag.as_ptr() as *const c_char,
            msg.as_ptr() as *const c_char,
        )
    };
}


#[link(name = "log")]
extern "C" {
    pub fn __android_log_write(prio: c_int,
                               tag: *const c_char,
                               text: *const c_char)
                               -> c_int;
    pub fn __android_log_buf_write(buf_id: c_int,
                                   prio: c_int,
                                   tag: *const c_char,
                                   text: *const c_char)
                                   -> c_int;
    pub fn __android_log_print(prio: c_int,
                               tag: *const c_char,
                               fmt: *const c_char,
                               ...)
                               -> c_int;
    pub fn __android_log_vprint(prio: c_int,
                                tag: *const c_char,
                                fmt: *const c_char,
                                ap: *mut c_va_list)
                                -> c_int;
    pub fn __android_log_assert(cond: *const c_char,
                                tag: *const c_char,
                                fmt: *const c_char,
                                ...);
    pub fn __android_log_is_loggable(prio: c_int,
                                     tag: *const c_char,
                                     default_prio: c_int)
                                     -> c_int;
    pub fn __android_log_write_log_message(log_message: *mut __android_log_message);
}