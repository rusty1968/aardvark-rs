use aardvark::aa_find_devices;
// Example using AA_find_devices
use aardvark_ffi as aardvark;

fn main() {
    let num_devices: i32 = 0;
    let mut devices: [u16; 16] = [0; 16];
    let count = unsafe { aa_find_devices(num_devices, devices.as_mut_ptr()) };
    println!("Found {} devices", count);
}
