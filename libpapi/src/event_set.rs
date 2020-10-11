use std::os::raw::c_int;

use libpapi_sys::*;

use super::Event;
use super::PapiError;

pub struct EventSet {
    raw_event_set: c_int,
}

impl EventSet {
    #[inline]
    pub fn new() -> Result<Self, PapiError> {
        let mut event_set = PAPI_NULL;
        let retval = unsafe { PAPI_create_eventset(&mut event_set) };

        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(EventSet {
                raw_event_set: event_set,
            })
        }
    }

    #[inline]
    pub fn insert(&self, event: Event) -> Result<(), PapiError> {
        let retval = unsafe { PAPI_add_event(self.raw_event_set, event.get_code()) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn assign_component(&self, component: i32) -> Result<(), PapiError> {
        let retval = unsafe { PAPI_assign_eventset_component(self.raw_event_set, component) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn remove(&mut self, event: Event) -> Result<(), PapiError> {
        let retval = unsafe { PAPI_remove_event(self.raw_event_set, event.get_code()) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn empty(&mut self) -> Result<(), PapiError> {
        let retval = unsafe { PAPI_cleanup_eventset(self.raw_event_set) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(())
        }
    }

    #[inline]
    pub fn raw_event_set(&self) -> i32 {
        self.raw_event_set
    }

    #[inline]
    pub fn size(&self) -> usize {
        unsafe { PAPI_num_events(self.raw_event_set) as usize }
    }
}

impl Drop for EventSet {
    fn drop(&mut self) {
        self.empty().unwrap();

        let retval = unsafe { PAPI_destroy_eventset(&mut self.raw_event_set as *mut i32) };
        if retval != PAPI_OK as i32 {
            panic!("Error while destroying EventSet: {}", retval)
        }
    }
}
