use super::{EventSet, PapiError, PapiCounter, Papi};

use libpapi_sys::*;
use std::sync::TryLockError;

pub struct PapiHandler<'a, 'b> {
    event_set: &'a EventSet,
    instance: &'b Papi
}

impl<'a, 'b> PapiHandler<'a, 'b> {
    pub fn new(event_set: &'a EventSet, papi: &'b Papi) -> PapiHandler<'a, 'b> {
        PapiHandler {
            event_set,
            instance: papi
        }
    }

    pub fn start_count(&self) -> Result<PapiCounter, PapiError> {
        let guard = match self.instance.single_counter_lock.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::Poisoned(_)) => panic!("Lock was poisoned"),
            Err(TryLockError::WouldBlock) => panic!("Multiple counter is not supported now!")
        };

        let retval = unsafe {PAPI_start(self.event_set.raw_event_set())};
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(PapiCounter {
                event_set: self.event_set,
                _guard: guard,
            })
        }
    }
}

impl Drop for PapiHandler<'_, '_> {
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