use aardvark_ffi as aardvark;

fn main() {
    let result = aardvark::find_aardvark_devices();
    if result.is_ok() {
        let devices = result.unwrap();
        for device in devices {
            println!("Unused device : {}", device)
        }
    }
}
