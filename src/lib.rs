#![feature(optin_builtin_traits)]

mod event_set;
mod error;
mod event;

pub use error::*;
pub use event_set::*;
pub use event::*;

use libpapi_sys::*;

pub struct Papi {}

impl Papi {
    pub fn new() -> Result<Papi, PapiError> {
        let retval = unsafe {PAPI_library_init(PAPI_VER_CURRENT)};
        if retval != PAPI_VER_CURRENT {
            return Err(PapiError::from(retval))
        } else {
            Ok(Papi {})
        }
    }

    pub fn start_count(&self, event_set: EventSet) -> Result<PapiCounter, PapiError> {
        let retval = unsafe {PAPI_start(event_set.raw_event_set())};
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(PapiCounter {
                event_set
            })
        }
    }
}

pub struct PapiCounter {
    event_set: EventSet
}

impl !Sync for PapiCounter {}
impl !Send for PapiCounter {}

impl PapiCounter {
    pub fn stop(&mut self, events: &[Event]) -> Result<Vec<i64>, PapiError> {
        let mut drain_event_set = EventSet::new()?;
        for event in events {
            self.event_set.remove(event.clone())?;
            drain_event_set.insert(event.clone())?;
        }

        let mut values = Vec::with_capacity(drain_event_set.size());
        let retval = unsafe {PAPI_stop(drain_event_set.raw_event_set(), values.as_mut_ptr())};
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(values)
        }
    }

    pub fn read_events(&self, events: &EventSet) -> Result<Vec<i64>, PapiError> {
        let mut values = Vec::with_capacity(events.size());

        let retval = unsafe {PAPI_read(events.raw_event_set(), values.as_mut_ptr())};
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(values)
        }
    }

    pub fn read(&self) -> Result<Vec<i64>, PapiError> {
        self.read_events(&self.event_set)
    }
}

impl Drop for PapiCounter {
    fn drop(&mut self) {
        let mut values = Vec::with_capacity(self.event_set.size());

        let retval = unsafe {PAPI_stop(self.event_set.raw_event_set(), values.as_mut_ptr())};

        if retval != PAPI_OK as i32 {
            panic!("Error while dropping events.Errno: {}", retval)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
