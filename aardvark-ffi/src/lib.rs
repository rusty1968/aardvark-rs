#![allow(dead_code)]
pub mod error;
pub mod i2c;
pub mod plugin;

pub use plugin::{Aardvark, AardvarkApi, AardvarkError};

pub type AardvarkResult<T> = Result<T, AardvarkError>;

#[derive(Clone, Debug)]
pub struct AardvarkHandle(Aardvark);

use libc::c_int;
use plugin::{AardvarkI2cFlags, AARDVARK_LIB, AA_PORT_NOT_FREE};

pub fn aa_open(device: u16) -> AardvarkResult<AardvarkHandle> {
    let api = unsafe { plugin::AardvarkApi::try_load(AARDVARK_LIB).unwrap() };
    let status = api.aa_open(device as c_int);
    if status < 0 {
        return Err(AardvarkError::new(status));
    }
    Ok(AardvarkHandle(status))
}

pub fn aa_close(device: u16) -> AardvarkResult<()> {
    let api = unsafe { plugin::AardvarkApi::try_load(AARDVARK_LIB).unwrap() };
    let status = api.aa_open(device as c_int);
    if status < 0 {
        return Err(AardvarkError::new(status));
    }
    Ok(())
}

impl AardvarkHandle {
    pub fn aa_i2c_write(
        &self,
        slave_addr: u16,
        flags: AardvarkI2cFlags,
        data_out: &[u8],
    ) -> AardvarkResult<()> {
        let api = unsafe { plugin::AardvarkApi::try_load(AARDVARK_LIB).unwrap() };

        let status = api.aa_i2c_write(
            self.0,
            slave_addr,
            flags,
            data_out.len() as u16,
            data_out.as_ptr(),
        );

        if status < 0 {
            return Err(AardvarkError::new(status));
        }
        Ok(())
    }

    pub fn aa_i2c_read(
        &self,
        slave_addr: u16,
        flags: AardvarkI2cFlags,
        data_in: &mut [u8],
    ) -> AardvarkResult<()> {
        let api = unsafe { plugin::AardvarkApi::try_load(AARDVARK_LIB).unwrap() };
        let status = api.aa_i2c_read(
            self.0,
            slave_addr,
            flags,
            data_in.len() as u16,
            data_in.as_mut_ptr(),
        );
        if status < 0 {
            return Err(AardvarkError::new(status));
        }
        Ok(())
    }
    pub fn aa_i2c_bitrate(&self, freq_khz: i32) -> AardvarkResult<()> {
        let api = unsafe { plugin::AardvarkApi::try_load(AARDVARK_LIB).unwrap() };

        let status = api.aa_i2c_bitrate(self.0, freq_khz);

        if status < 0 {
            return Err(AardvarkError::new(status));
        }
        Ok(())
    }
}

pub fn find_aardvark_devices() -> AardvarkResult<Vec<u16>> {
    let api = unsafe { plugin::AardvarkApi::try_load(AARDVARK_LIB).unwrap() };
    let mut devices: [u16; 16] = [0; 16];
    let num_devices = api.aa_find_devices(devices.len() as i32, devices.as_mut_ptr());

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

pub fn find_and_open_first_unused_device() -> AardvarkResult<AardvarkHandle> {
    let devices = find_unused_aardvark_devices()?;

    if let Some(device) = devices.first() {
        return aa_open(*device);
    }
    Err(AardvarkError::UNABLE_TO_FIND_UNUSED_DEVICE)
}
