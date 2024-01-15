pub mod error;
pub mod i2c;

pub use error::{AardvarkError, AardvarkResult};

use aardvark_ffi::{aa_find_devices, Aardvark, AardvarkConfig_AA_CONFIG_SPI_I2C, AA_PORT_NOT_FREE};
use i2c::{aardvark_configure, aardvark_i2_bitrate, aardvark_open, I2CDevice};

// Returns array with the port numbers.
fn find_aardvark_devices() -> AardvarkResult<Vec<u16>> {
    let mut devices: [u16; 16] = [0; 16];
    let num_devices = unsafe { aa_find_devices(devices.len() as i32, devices.as_mut_ptr()) };

    if num_devices < 0 {
        // Return empty vector if no devices are found
        return Err(AardvarkError::new(num_devices));
    }

    let num_devices = (num_devices as usize).min(devices.len());
    // Truncate array to number of devices found or the size of the devices array
    Ok(devices[0..num_devices].to_vec())
}

pub fn find_unused_aardvark_devices() -> AardvarkResult<Vec<u16>> {
    let devices = find_aardvark_devices()?;
    let unused_devices = devices
        .into_iter()
        .filter(|device| (*device & AA_PORT_NOT_FREE as u16) == 0);
    Ok(unused_devices.collect())
}

pub fn find_and_open_first_unused_device() -> AardvarkResult<Aardvark> {
    let devices = find_unused_aardvark_devices()?;

    if let Some(device) = devices.first() {
        aardvark_open(*device as i32)?;
    }
    Err(AardvarkError::UNABLE_TO_FIND_UNUSED_DEVICE)
}

pub fn open_i2c(bitrate_khz: i32) -> AardvarkResult<I2CDevice> {
    let aardvark = find_and_open_first_unused_device()?;

    aardvark_configure(aardvark, AardvarkConfig_AA_CONFIG_SPI_I2C)?;

    aardvark_i2_bitrate(aardvark, bitrate_khz)?;

    Ok(I2CDevice::new(aardvark))
}
