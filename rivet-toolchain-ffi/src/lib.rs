mod util;

use std::{
	ffi::{CStr, CString},
	os::raw::c_char,
};

#[no_mangle]
pub extern "C" fn run_task(
	run_config: *const c_char,
	name: *const c_char,
	input_json: *const c_char,
) -> *mut c_char {
	let run_config = unsafe { CStr::from_ptr(run_config).to_str().unwrap() };
	let name = unsafe { CStr::from_ptr(name).to_str().unwrap() };
	let input_json = unsafe { CStr::from_ptr(input_json).to_str().unwrap() };

	let result = util::run_task(
		run_config.to_string(),
		name.to_string(),
		input_json.to_string(),
	);

	let c_result = CString::new(result).unwrap();
	c_result.into_raw()
}

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
	unsafe {
		if s.is_null() {
			return;
		}
		let _ = CString::from_raw(s);
	};
}
