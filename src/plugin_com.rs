// This file is a literal translation of the CFPluginCOM.h header, which has this issue.
#![allow(overflowing_literals)]

use libc::c_void;
use ::{CFUUIDBytes, SInt32, UInt32};

pub type HRESULT = SInt32;
pub type ULONG = UInt32;
pub type LPVOID = *mut c_void;
pub type REFIID = CFUUIDBytes;

#[macro_export]
macro_rules! SUCCEEDED {
    ($Status:expr) => ($Status as $crate::HRESULT >= 0);
}

#[macro_export]
macro_rules! FAILED {
    ($Status:expr) => ($Status as $crate::HRESULT < 0);
}

#[macro_export]
macro_rules! IS_ERROR {
    ($Status:expr) => ($Status as $crate::c_ulong >> 31 == $crate::SEVERITY_ERROR);
}

pub const SEVERITY_SUCCESS: HRESULT = 0;
pub const SEVERITY_FAILURE: HRESULT = 1;

#[macro_export]
macro_rules! HRESULT_CODE {
    ($hr:expr) => (($hr as $crate::HRESULT) & 0xFFFF);
}

#[macro_export]
macro_rules! HRESULT_FACILITY {
    ($hr:expr) => ((($hr as $crate::HRESULT) >> 16) & 0x1FFF);
}

#[macro_export]
macro_rules! HRESULT_SEVERITY {
    ($hr:expr) => ((($hr as $crate::HRESULT) >> 31) & 0x1);
}

#[macro_export]
macro_rules! MAKE_HRESULT {
    ($sev:expr, $fac:expr, $code:expr) => (
        ((($sev as ::c_ulong) << 31) | (($fac as $crate::c_ulong) << 16) | ($code as $crate::c_ulong)) as $crate::HRESULT
    );
}

pub const S_OK: HRESULT = 0x00000000;
pub const S_FALSE: HRESULT = 0x00000001;

pub const E_UNEXPECTED: HRESULT = 0x8000FFFF;
pub const E_NOTIMPL: HRESULT = 0x80000001;
pub const E_OUTOFMEMORY: HRESULT = 0x80000002;
pub const E_INVALIDARG: HRESULT = 0x80000003;
pub const E_NOINTERFACE: HRESULT = 0x80000004;
pub const E_POINTER: HRESULT = 0x80000005;
pub const E_HANDLE: HRESULT = 0x80000006;
pub const E_ABORT: HRESULT = 0x80000007;
pub const E_FAIL: HRESULT = 0x80000008;
pub const E_ACCESSDENIED: HRESULT = 0x80000009;

macro_rules! IUnknownUUID {
    () => (
        $crate::CFUUIDGetConstantUUIDWithBytes($crate::kCFAllocatorSystemDefault,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)
    );
}

#[cfg(not(target_os = "Windows"))]
#[repr(C)]
pub struct IUNKNOWN_C_GUTS {
    pub _reserved: *mut c_void,
    pub QueryInterface: extern "C" fn(thisPointer: *mut c_void, iid: REFIID, ppv: *mut LPVOID) -> HRESULT,
    pub AddRef: extern "C" fn(thisPointer: *mut c_void) -> ULONG,
    pub Release: extern "C" fn(thisPointer: *mut c_void) -> ULONG
}

#[cfg(target_os = "Windows")]
#[repr(C)]
pub struct IUNKNOWN_C_GUTS {
    pub _reserved: *mut c_void,
    pub QueryInterface: extern "stdcall" fn(thisPointer: *mut c_void, iid: REFIID, ppv: *mut LPVOID) -> HRESULT,
    pub AddRef: extern "stdcall" fn(thisPointer: *mut c_void) -> ULONG,
    pub Release: extern "stdcall" fn(thisPointer: *mut c_void) -> ULONG
}

#[repr(C)]
pub struct IUnknownVTbl {
    pub vtbl: IUNKNOWN_C_GUTS
}
