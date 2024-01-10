use crate::plugin::AardvarkError;
use crate::AardvarkHandle;
use embedded_hal::i2c::ErrorType;
use embedded_hal::i2c::{I2c, Operation as I2cOperation, SevenBitAddress, TenBitAddress};
use std::fmt;
use std::ops::Deref;
pub struct I2CDevice {
    handle: AardvarkHandle,
}

impl I2CDevice {
    pub fn new(handle: AardvarkHandle) -> Self {
        Self { handle }
    }
}

impl std::ops::Deref for I2CDevice {
    type Target = AardvarkHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl std::ops::DerefMut for I2CDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handle
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
                    self.deref()
                        .aa_i2c_read(address, 0, buffer)
                        .map_err(|err| I2CError(err))?;
                    println!("Read")
                }
                I2cOperation::Write(bytes) => {
                    self.deref()
                        .aa_i2c_write(address, 0, bytes)
                        .map_err(|err| I2CError(err))?;
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
        I2c::<TenBitAddress>::transaction(self, u16::from(address), operations)
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
