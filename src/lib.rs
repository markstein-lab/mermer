#[no_mangle]
pub extern "C" fn ffi_test(x: i32) -> i32 {
    x * 2
}
