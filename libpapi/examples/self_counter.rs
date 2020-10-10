use std::convert::TryFrom;

use libpapi::{Event, EventSet, Papi};

fn main() {
    let instance = Papi::new().unwrap();

    let event_set = EventSet::new().unwrap();
    let event = Event::try_from("PAPI_TOT_INS").unwrap();
    event_set.insert(event).unwrap();

    let handler = instance.local_handler(&event_set);
    let counter = handler.start_count().unwrap();

    loop {
        let values = counter.read().unwrap();
        dbg!(values);
    }
}
