/*=========================================================================
| (c) 2004-2007  Total Phase, Inc.
|--------------------------------------------------------------------------
| Project : Aardvark Sample Code
| File    : aalights.c
|--------------------------------------------------------------------------
| Flash the lights on the Aardvark I2C/SPI Activity Board.
|--------------------------------------------------------------------------
| Redistribution and use of this file in source and binary forms, with
| or without modification, are permitted.
|
| THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
| "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
| LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
| FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE
| COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
| INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
| BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
| LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
| CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
| LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
| ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
| POSSIBILITY OF SUCH DAMAGE.
 ========================================================================*/

//=========================================================================
// INCLUDES
//=========================================================================

use std::ffi::CStr;

use aardvark_ffi::{
    aa_close, aa_configure, aa_i2c_bitrate, aa_i2c_pullup, aa_i2c_write, aa_open, aa_sleep_ms,
    aa_status_string, aa_target_power, Aardvark, AardvarkConfig_AA_CONFIG_SPI_I2C,
    AardvarkI2cFlags_AA_I2C_NO_FLAGS, AA_I2C_PULLUP_BOTH, AA_TARGET_POWER_BOTH,
};

//=========================================================================
// CONSTANTS
//=========================================================================
pub const I2C_BITRATE: i32 = 100; // kHz

//=========================================================================
// STATIC FUNCTIONS
//=========================================================================
fn flash_lights(handle: Aardvark) -> i32 {
    let mut data_out: [u8; 16] = [0; 16];

    // Configure I/O expander lines as outputs
    data_out[0] = 0x03;
    data_out[1] = 0x00;
    let res = unsafe { aa_i2c_write(handle, 0x38, AardvarkI2cFlags_AA_I2C_NO_FLAGS, 2, data_out.as_mut_ptr()) };
    if res < 0 {
        return res;
    }

    if res == 0 {
        println!("error: slave device 0x38 not found\n");
        return 0;
    }

    // Turn lights on in sequence
    let i = 0xff;
    while i > 0 {
        let i = (i << 1) & 0xff;
        data_out[0] = 0x01;
        data_out[1] = i;
        let res =
            unsafe { aa_i2c_write(handle, 0x38, AardvarkI2cFlags_AA_I2C_NO_FLAGS, 2, data_out.as_mut_ptr()) };
        if res < 0 {
            return res;
        }
        unsafe { aa_sleep_ms(70) };
    }

    // Leave lights on for 100 ms
    unsafe { aa_sleep_ms(100) };

    // Turn lights off in sequence
    let i = 0x00;
    while i != 0xff {
        let i = (i << 1) | 0x01;
        data_out[0] = 0x01;
        data_out[1] = i;
        let res = unsafe { aa_i2c_write(handle, 0x38, AardvarkI2cFlags_AA_I2C_NO_FLAGS, 2, data_out.as_mut_ptr()) };
        if res < 0 {
            return res;
        }
        unsafe { aa_sleep_ms(70) };
    }

    unsafe { aa_sleep_ms(100) };

    // Configure I/O expander lines as inputs
    data_out[0] = 0x03;
    data_out[1] = 0xff;
    let res = unsafe { aa_i2c_write(handle, 0x38, AardvarkI2cFlags_AA_I2C_NO_FLAGS, 2, data_out.as_mut_ptr()) };
    if res < 0 {
        return res;
    }

    return 0;
}

//=========================================================================
// MAIN PROGRAM
//=========================================================================
fn main() {
    let port = std::env::args().nth(1).expect("usage: aalights PORT");
    let port = port.parse::<i32>().unwrap();

    // Open the device
    let handle = unsafe { aa_open(port) };
    if handle <= 0 {
        println!("Unable to open Aardvark device on port {port}");
        println!("Error code = {handle}");
        return;
    }

    // Ensure that the I2C subsystem is enabled
    unsafe { aa_configure(handle, AardvarkConfig_AA_CONFIG_SPI_I2C) };

    // Enable the I2C bus pullup resistors (2.2k resistors).
    // This command is only effective on v2.0 hardware or greater.
    // The pullup resistors on the v1.02 hardware are enabled by default.
    unsafe { aa_i2c_pullup(handle, AA_I2C_PULLUP_BOTH as u8) };

    // Power the board using the Aardvark adapter's power supply.
    // This command is only effective on v2.0 hardware or greater.
    // The power pins on the v1.02 hardware are not enabled by default.
    unsafe { aa_target_power(handle, AA_TARGET_POWER_BOTH as u8) };

    // Set the bitrate
    let bitrate = unsafe { aa_i2c_bitrate(handle, I2C_BITRATE) };
    println!("Bitrate set to {bitrate} kHz");

    let res = flash_lights(handle);
    if res < 0 {
        let char_ptr = unsafe { aa_status_string(res) };
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        println!("error: {}", c_str.to_str().unwrap());
    }

    // Close the device and exit
    unsafe { aa_close(handle) };

    return;
}
