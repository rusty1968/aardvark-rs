use aardvark_rs::find_and_open_first_unused_device;
use aardvark_rs::i2c::I2CDevice;

use embedded_hal::i2c::I2c;

use anyhow::Result;

use aardvark_ffi::aa_configure;

use aardvark_ffi::{
    aa_i2c_bitrate, aa_i2c_pullup, aa_sleep_ms,
    aa_target_power, AardvarkConfig_AA_CONFIG_SPI_I2C,
    AA_I2C_PULLUP_BOTH, AA_TARGET_POWER_BOTH,
};

//=========================================================================
// CONSTANTS
//=========================================================================
pub const I2C_BITRATE: i32 = 100; // kHz

fn flash_lights(device: &mut I2CDevice) -> Result<()> {
    let mut data_out: [u8; 2] = [0; 2];

    // Configure I/O expander lines as outputs
    data_out[0] = 0x03;
    data_out[1] = 0x00;

    let mut v = vec![];

    v.extend_from_slice(&data_out);

    device.write(0x38_u8, &v)?;

    // Turn lights on in sequence
    println!("Turn lights on in sequence");
    let mut i = 0xff;
    while i > 0 {
        i = (i << 1) & 0xff;

        data_out[0] = 0x01;
        data_out[1] = i;

        let mut v = vec![];

        v.extend_from_slice(&data_out);
    
        device.write(0x38_u8, &v)?;
    
        unsafe { aa_sleep_ms(70) };
    }

    // Leave lights on for 100 ms
    println!("Leave lights on for 100ms");
    unsafe { aa_sleep_ms(100) };

    // Turn lights off in sequence
    println!("Turn lights off in sequence");
    let mut i = 0x00;
    while i != 0xff {
        i = (i << 1) | 0x01;
        data_out[0] = 0x01;
        data_out[1] = i;

        let mut v = vec![];

        v.extend_from_slice(&data_out);
    
        device.write(0x38_u8, &v)?;

        unsafe { aa_sleep_ms(70) };
    }

    unsafe { aa_sleep_ms(100) };

   // Configure I/O expander lines as inputs
   data_out[0] = 0x03;
   data_out[1] = 0xff;
   let mut v = vec![];

   v.extend_from_slice(&data_out);

   device.write(0x38_u8, &v)?;


    Ok(())
}

pub fn main() {
    let aardvark = find_and_open_first_unused_device().expect("Can't open Aardvark device");

    // Ensure that the I2C subsystem is enabled
    unsafe { aa_configure(aardvark, AardvarkConfig_AA_CONFIG_SPI_I2C) };

    // Enable the I2C bus pullup resistors (2.2k resistors).
    // This command is only effective on v2.0 hardware or greater.
    // The pullup resistors on the v1.02 hardware are enabled by default.
    unsafe { aa_i2c_pullup(aardvark, AA_I2C_PULLUP_BOTH as u8) };

    // Power the board using the Aardvark adapter's power supply.
    // This command is only effective on v2.0 hardware or greater.
    // The power pins on the v1.02 hardware are not enabled by default.
    unsafe { aa_target_power(aardvark, AA_TARGET_POWER_BOTH as u8) };

    // Set the bitrate
    let bitrate = unsafe { aa_i2c_bitrate(aardvark, I2C_BITRATE) };
    println!("Bitrate set to {bitrate} kHz");

    let mut i2c = I2CDevice::new(aardvark);

    flash_lights(&mut i2c).expect("Cant flash lights");
}
