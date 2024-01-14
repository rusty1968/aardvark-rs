use crate::AardvarkError;
use aardvark_ffi::AardvarkI2cFlags;
use aardvark_ffi::AardvarkI2cFlags_AA_I2C_10_BIT_ADDR;
use aardvark_ffi::AardvarkI2cFlags_AA_I2C_NO_FLAGS;
use aardvark_ffi::{aa_i2c_read, aa_i2c_write, Aardvark};
use embedded_hal::i2c::ErrorType;
use embedded_hal::i2c::{I2c, Operation as I2cOperation, SevenBitAddress, TenBitAddress};
use std::fmt;

pub struct I2CDevice {
    handle: Aardvark,
}

impl I2CDevice {
    pub fn new(handle: Aardvark) -> Self {
        Self { handle }
    }
}

impl I2c<TenBitAddress> for I2CDevice {
    fn transaction(
        &mut self,
        address: u16,
        operations: &mut [I2cOperation],
    ) -> Result<(), Self::Error> {
        for (_, operation) in operations.iter_mut().enumerate() {
            match operation {
                I2cOperation::Read(buffer) => {
                    aardvark_i2c_read(
                        self.handle,
                        address,
                        AardvarkI2cFlags_AA_I2C_10_BIT_ADDR,
                        buffer,
                    )
                    .map_err(I2CError)?;
                }
                I2cOperation::Write(buffer) => {
                    aardvark_i2c_write(
                        self.handle,
                        address,
                        AardvarkI2cFlags_AA_I2C_10_BIT_ADDR,
                        buffer,
                    )
                    .map_err(I2CError)?;
                }
            }
        }
        Ok(())
    }
}
impl I2c<SevenBitAddress> for I2CDevice {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [I2cOperation],
    ) -> Result<(), Self::Error> {
        for (_, operation) in operations.iter_mut().enumerate() {
            match operation {
                I2cOperation::Read(buffer) => {
                    aardvark_i2c_read(
                        self.handle,
                        address as u16,
                        AardvarkI2cFlags_AA_I2C_NO_FLAGS,
                        buffer,
                    )
                    .map_err(I2CError)?;
                }
                I2cOperation::Write(buffer) => {
                    aardvark_i2c_write(
                        self.handle,
                        address as u16,
                        AardvarkI2cFlags_AA_I2C_NO_FLAGS,
                        buffer,
                    )
                    .map_err(I2CError)?;
                }
            }
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct I2CError(AardvarkError);

impl fmt::Display for I2CError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for I2CError {}

impl ErrorType for I2CDevice {
    type Error = I2CError;
}

impl embedded_hal::i2c::Error for I2CError {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        embedded_hal::i2c::ErrorKind::Other
    }
}

pub fn aardvark_i2c_read(
    aardvark: Aardvark,
    address: u16,
    flags: AardvarkI2cFlags,
    buffer: &mut [u8],
) -> Result<(), AardvarkError> {
    let status = unsafe {
        aa_i2c_read(
            aardvark,
            address,
            flags,
            buffer.len() as u16,
            buffer.as_mut_ptr(),
        )
    };

    if status != 0 {
        let error = AardvarkError::new(status);
        return Err(error);
    }
    Ok(())
}

pub fn aardvark_i2c_write(
    aardvark: Aardvark,
    address: u16,
    flags: AardvarkI2cFlags,
    buffer: &[u8],
) -> Result<(), AardvarkError> {
    let status = unsafe {
        aa_i2c_write(
            aardvark,
            address,
            flags,
            buffer.len() as u16,
            buffer.as_ptr(),
        )
    };

    if status != 0 {
        let error = AardvarkError::new(status);
        return Err(error);
    }
    Ok(())
}
