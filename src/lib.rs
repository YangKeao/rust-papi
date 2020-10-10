#![feature(optin_builtin_traits)]
#![feature(negative_impls)]

mod counter;
mod error;
mod event;
mod event_set;
mod handler;

pub use self::counter::*;
pub use self::error::*;
pub use self::event::*;
pub use self::event_set::*;
pub use self::handler::*;

use std::sync::Mutex;

use libpapi_sys::*;

pub struct Papi {
    single_counter_lock: Mutex<()>,
}

impl Papi {
    pub fn new() -> Result<Papi, PapiError> {
        let retval = unsafe { PAPI_library_init(PAPI_VER_CURRENT) };
        if retval != PAPI_VER_CURRENT {
            Err(PapiError::from(retval))
        } else {
            Ok(Papi {
                single_counter_lock: Mutex::new(()),
            })
        }
    }

    pub fn local_handler<'a>(&self, event_set: &'a EventSet) -> PapiHandler<'a, '_> {
        PapiHandler::new(event_set, self)
    }

    pub fn attach<'a, 'b>(
        &'b self,
        event_set: &'a EventSet,
        tid: u64,
    ) -> Result<PapiHandler<'a, 'b>, PapiError> {
        let retval = unsafe {
            let mut option_t = PAPI_option_t {
                attach: PAPI_attach_option_t {
                    eventset: event_set.raw_event_set(),
                    tid,
                },
            };
            PAPI_set_opt(PAPI_ATTACH as i32, &mut option_t)
        };
        if retval != PAPI_OK as i32 {
            Err(PapiError::from(retval))
        } else {
            Ok(PapiHandler::new(event_set, self))
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
