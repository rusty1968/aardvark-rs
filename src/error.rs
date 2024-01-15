use std::ffi::CStr;
use std::fmt;
use std::{num::NonZeroI32, os::raw::c_int};

use aardvark_ffi::aa_status_string;

#[derive(Debug)]
pub struct AardvarkError(std::num::NonZeroI32);

pub type AardvarkResult<T> = std::result::Result<T, AardvarkError>;

impl std::error::Error for AardvarkError {}
impl AardvarkError {
    pub const fn new_from_const(status: c_int) -> Self {
        match NonZeroI32::new(status) {
            Some(val) => Self(val),
            None => panic!("AardvarkError cannot be 0"),
        }
    }
    pub fn new(status: c_int) -> Self {
        match NonZeroI32::new(status) {
            Some(val) => Self(val),
            None => panic!("AardvarkError cannot be 0"),
        }
    }
    pub const UNABLE_TO_FIND_UNUSED_DEVICE: AardvarkError = AardvarkError::new_from_const(0x0001);
}
impl From<AardvarkError> for core::num::NonZeroI32 {
    fn from(val: AardvarkError) -> Self {
        val.0
    }
}
impl From<AardvarkError> for i32 {
    fn from(val: AardvarkError) -> Self {
        core::num::NonZeroI32::from(val).get()
    }
}

impl AardvarkError {
    pub fn get_aardvark_status_string(
        error: AardvarkError,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let cstr = unsafe { CStr::from_ptr(aa_status_string(error.0.get() as c_int)) };

        match cstr.to_str() {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(From::from(e)),
        }
    }
}

impl fmt::Display for AardvarkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Aardvark error: {}", self.0.get().to_string().as_str())
    }
}
