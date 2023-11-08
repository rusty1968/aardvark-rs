use aardvark::find_and_open_first_unused_device;
use aardvark_ffi as aardvark;
pub use i2c::I2CDevice;

use anyhow::Result;

pub mod i2c;

pub fn open_i2c() -> Result<i2c::I2CDevice> {
    let aardvark = find_and_open_first_unused_device()?;

    aardvark.aa_i2c_bitrate(400)?;

    Ok(I2CDevice::new(aardvark))
}
