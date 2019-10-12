#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(env!("BINDING_PATH"));

pub const PAPI_VER_CURRENT: std::os::raw::c_int = 84344832;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let mut event_set = PAPI_NULL;
            let mut values = [0i64];

            let retval = PAPI_library_init(PAPI_VER_CURRENT);
            if retval != PAPI_VER_CURRENT {
                panic!("PAPI_library_init failed {}", retval)
            }

            let mut PAPI_TOT_INS: std::os::raw::c_int = 0;
            let retval = PAPI_event_name_to_code(std::ffi::CStr::from_bytes_with_nul_unchecked(b"PAPI_TOT_INS\0").as_ptr(), &mut PAPI_TOT_INS);
            if retval != PAPI_OK as i32 {
                panic!("PAPI_event_name_to_code failed {}", retval)
            }

            let retval = PAPI_create_eventset(&mut event_set);
            if retval != PAPI_OK as i32 {
                panic!("PAPI_create_eventset error {}", retval)
            }

            let retval = PAPI_add_event(event_set.clone(), PAPI_TOT_INS as i32);
            if retval != PAPI_OK as i32 {
                panic!("PAPI_add_event failed {}", retval)
            }

            let retval = PAPI_start(event_set.clone());
            if retval != PAPI_OK as i32 {
                panic!("PAPI_start failed {}", retval)
            }

            let mut sum = 0;
            for i in 0..1000000 {
                sum *= i;
            }

            let retval = PAPI_read(event_set.clone(), values.as_mut_ptr());
            if retval != PAPI_OK as i32{
                panic!("PAPI_read failed")
            }

            let retval = PAPI_stop(event_set, values.as_mut_ptr());
            if retval != PAPI_OK as i32{
                panic!("PAPI_stop failed")
            }

            println!("value: {}", values[0])
        }
    }
}
