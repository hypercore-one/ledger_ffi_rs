use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr::null,
};

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ExecutionResult<T>
where
    T: Serialize,
{
    Ok(T),
    Err(String),
}

pub trait ToCStringPtr {
    fn to_cstring_ptr(self) -> *mut c_char;
}

impl ToCStringPtr for String {
    fn to_cstring_ptr(self) -> *mut c_char {
        CString::new(self).unwrap().into_raw()
    }
}

pub trait ToOptionalCStringPtr {
    fn to_optional_cstring_ptr(self) -> *mut c_char;
}

impl ToOptionalCStringPtr for Option<String> {
    fn to_optional_cstring_ptr(self) -> *mut c_char {
        match self {
            Some(string) => string.to_cstring_ptr(),
            None => null::<c_char>() as *mut c_char,
        }
    }
}

pub trait ToStringFromPtr {
    unsafe fn to_string_from_ptr(self) -> String;
}

impl ToStringFromPtr for *mut c_char {
    unsafe fn to_string_from_ptr(self) -> String {
        CStr::from_ptr(self).to_str().unwrap().to_owned()
    }
}

pub trait ToOptionalStringFromPtr {
    unsafe fn to_optional_string_from_ptr(self) -> Option<String>;
}

impl ToOptionalStringFromPtr for *mut c_char {
    unsafe fn to_optional_string_from_ptr(self) -> Option<String> {
        match !self.is_null() {
            true => Some(self.to_string_from_ptr()),
            false => None,
        }
    }
}

pub trait HandleError {
    type Output;

    fn handle_error(self) -> Result<Self::Output, String>;
}

impl<T, E> HandleError for Result<T, E>
where
    E: ToString,
{
    type Output = T;

    fn handle_error(self) -> Result<Self::Output, String> {
        self.map_err(|e| e.to_string())
    }
}

pub trait MatchResult {
    fn match_result(self) -> *mut c_char;
}

impl<T> MatchResult for Result<T, String>
where
    T: Serialize,
{
    fn match_result(self) -> *mut c_char {
        let result = match self {
            Ok(ok) => ExecutionResult::Ok(ok),
            Err(err) => ExecutionResult::Err(err),
        };

        serde_json::to_string(&result).unwrap().to_cstring_ptr()
    }
}

pub trait ToNekoton<T> {
    fn to_nekoton(self) -> T;
}

pub trait ToSerializable<T> {
    fn to_serializable(self) -> T;
}
