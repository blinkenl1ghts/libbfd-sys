#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ffi::CString;

    #[test]
    fn open_bfd() {
        unsafe {
            bfd_init();
            let bfdh: *mut bfd = bfd_openr(CString::new("/bin/ls").unwrap().as_ptr(), std::ptr::null());
            if bfdh.is_null() {
                panic!("bfd_openr returned NULL!");
            }
            if bfd_check_format(bfdh, bfd_format_bfd_object) == 0 {
                panic!("bfd_check_format failed!");
            }
            panic!("Entry-point: 0x{:x}", (*bfdh).start_address);
        }
    }
}
