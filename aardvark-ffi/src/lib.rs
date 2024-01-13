#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_devices() {
        let num_devices = 16;
        let mut devices = [0_u16; 16];
        let mut unique_ids = [0_u32; 16];
        let count = unsafe {
            aa_find_devices_ext(
                num_devices,
                devices.as_mut_ptr(),
                devices.len() as i32,
                unique_ids.as_mut_ptr(),
            )
        };

        println!(" {} devices found", count);
    }
}