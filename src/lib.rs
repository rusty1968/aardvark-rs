use aardvark::find_and_open_first_unused_device;
pub use aardvark::i2c::I2CDevice;
use aardvark_ffi as aardvark;

use anyhow::Result;

pub fn open_i2c() -> Result<aardvark::i2c::I2CDevice> {
    let aardvark = find_and_open_first_unused_device()?;

    aardvark.aa_i2c_bitrate(400)?;

    Ok(I2CDevice::new(aardvark))
}
