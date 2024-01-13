use aardvark_ffi::Aardvark;

#[derive(Clone, Debug)]
pub struct AardvarkHandle(Aardvark);

pub mod error;
pub mod i2c;

pub use error::AardvarkError;
