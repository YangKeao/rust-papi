use std::convert::TryFrom;
use std::os::raw::c_int;

use super::PapiError;
use std::ffi::CString;

use libpapi_sys::*;

#[derive(Copy, Clone)]
pub struct Event(c_int);

impl Event {
    #[inline]
    pub fn get_code(self) -> c_int {
        self.0
    }
}

impl TryFrom<&str> for Event {
    type Error = PapiError;

    #[inline]
    fn try_from(event_name: &str) -> Result<Self, Self::Error> {
        let event_name = CString::new(event_name).expect("crate CString from event_name failed");

        let mut event: c_int = 0;
        let retval = unsafe { PAPI_event_name_to_code(event_name.into_raw(), &mut event) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(Event(event))
        }
    }
}
