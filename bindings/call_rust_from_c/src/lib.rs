use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn iadd(left: c_int, right: c_int) {
    println!("{}", left + right);
}

