#![feature(optin_builtin_traits)]

use std::sync::{Mutex, MutexGuard};

mod event_set;
mod error;
mod event;

pub use error::*;
pub use event_set::*;
pub use event::*;

use libpapi_sys::*;

pub struct Papi {
    single_counter_lock: Mutex<()>
}

impl Papi {
    pub fn new() -> Result<Papi, PapiError> {
        let retval = unsafe {PAPI_library_init(PAPI_VER_CURRENT)};
        if retval != PAPI_VER_CURRENT {
            return Err(PapiError::from(retval))
        } else {
            Ok(Papi {
                single_counter_lock: Mutex::new(())
            })
        }
    }

    pub fn start_count(&self, event_set: EventSet) -> Result<PapiCounter, PapiError> {
        let guard = match self.single_counter_lock.try_lock() {
            Ok(guard) => guard,
            Err(_) => panic!("Multiple counter is not supported now!")
        };

        let retval = unsafe {PAPI_start(event_set.raw_event_set())};
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(PapiCounter {
                event_set,
                _guard: guard,
            })
        }
    }

    pub fn attach(&self, event_set: EventSet, tid: u64) -> Result<AttachHandler, PapiError> {
        let retval = unsafe {
            let mut option_t = PAPI_option_t {
                attach: PAPI_attach_option_t {
                    eventset: event_set.raw_event_set(),
                    tid,
                }
            };
            PAPI_set_opt(PAPI_ATTACH as i32, &mut option_t)
        };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(AttachHandler::new(event_set))
        }
    }
}

pub struct PapiCounter<'a> {
    event_set: EventSet,
    _guard: MutexGuard<'a, ()>
}

impl !Sync for PapiCounter<'_> {}
impl !Send for PapiCounter<'_> {}

impl PapiCounter<'_> {
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

impl Drop for PapiCounter<'_> {
    fn drop(&mut self) {
        let mut values = Vec::with_capacity(self.event_set.size());

        let retval = unsafe {PAPI_stop(self.event_set.raw_event_set(), values.as_mut_ptr())};

        if retval != PAPI_OK as i32 {
            panic!("Error while dropping events.Errno: {}", retval)
        }
    }
}

pub struct AttachHandler {
    event_set: EventSet
}

impl AttachHandler {
    fn new(event_set: EventSet) -> AttachHandler {
        AttachHandler {
            event_set,
        }
    }
}

impl Drop for AttachHandler {
    fn drop(&mut self) {
        let retval = unsafe {
            let mut option_t = PAPI_option_t {
                attach: PAPI_attach_option_t {
                    eventset: self.event_set.raw_event_set(),
                    tid: 0,
                }
            };
            PAPI_set_opt(PAPI_DETACH as i32, &mut option_t)
        };
        if retval != PAPI_OK as i32 {
            panic!("Detach failed. Errno: {}", retval)
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
