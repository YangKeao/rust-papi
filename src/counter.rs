use super::EventSet;
use super::PapiError;

use libpapi_sys::*;

use std::sync::MutexGuard;

pub struct PapiCounter<'a, 'b> {
    pub event_set: &'a EventSet,
    pub _guard: MutexGuard<'b, ()>,
}

impl !Sync for PapiCounter<'_, '_> {}
impl !Send for PapiCounter<'_, '_> {}

impl PapiCounter<'_, '_> {
    pub fn read_events(&self, events: &EventSet) -> Result<Vec<i64>, PapiError> {
        let mut values = Vec::with_capacity(events.size());

        let retval = unsafe { PAPI_read(events.raw_event_set(), values.as_mut_ptr()) };
        unsafe { values.set_len(events.size()) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(values)
        }
    }

    pub fn read(&self) -> Result<Vec<i64>, PapiError> {
        self.read_events(&self.event_set)
    }

    pub fn read_and_reset_events(&self, events: &EventSet) -> Result<Vec<i64>, PapiError> {
        let mut values = vec![0; events.size()];

        let retval = unsafe { PAPI_accum(events.raw_event_set(), values.as_mut_ptr()) };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(values)
        }
    }

    pub fn read_and_reset(&self) -> Result<Vec<i64>, PapiError> {
        self.read_and_reset_events(&self.event_set)
    }
}

impl Drop for PapiCounter<'_, '_> {
    fn drop(&mut self) {
        let mut values = Vec::with_capacity(self.event_set.size());

        let retval = unsafe { PAPI_stop(self.event_set.raw_event_set(), values.as_mut_ptr()) };

        if retval != PAPI_OK as i32 {
            panic!("Error while dropping events.Errno: {}", retval)
        }
    }
}
