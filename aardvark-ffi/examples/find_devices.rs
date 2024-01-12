// Example using AA_find_devices
use aardvark_ffi as aardvark;

fn main() {
    let api = unsafe { aardvark::AardvarkApi::try_load("./dynamic-lib/aardvark.so").unwrap() };
    let num_devices: i32 = 16;
    let mut devices: [u16; 16] = [0; 16];
    let count = api.aa_find_devices(num_devices, devices.as_mut_ptr());
    println!("Found {} devices", count);
}
