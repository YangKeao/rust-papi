use std::convert::TryFrom;

use clap::{value_t, App, Arg};
use libc::pid_t;
use nix::sys::ptrace;
use nix::unistd::Pid;
use papi::{Event, EventSet, Papi};

fn main() {
    let matches = App::new("Counter Monitor")
        .version("0.1")
        .author("Yang Keao <keao.yang@yahoo.com>")
        .arg(
            Arg::with_name("events")
                .short("e")
                .long("events")
                .value_name("events")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tid")
                .short("t")
                .long("tid")
                .value_name("tid")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .value_name("interval")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    let events = matches.value_of("events").unwrap();
    let tid = value_t!(matches, "tid", u64).unwrap();
    let interval = value_t!(matches, "interval", u64).unwrap_or(1000);
    let interval = std::time::Duration::from_millis(interval);

    let instance = Papi::new().unwrap();

    let event_set = EventSet::new().unwrap();
    event_set.assign_component(0).unwrap();

    let pid = Pid::from_raw(tid as pid_t);
    ptrace::attach(pid).unwrap();
    nix::sys::wait::waitpid(pid, Some(nix::sys::wait::WaitPidFlag::WSTOPPED)).unwrap();

    let handler = instance.attach(&event_set, tid).unwrap();
    for event in events.split(',') {
        let event = Event::try_from(event).unwrap();
        event_set.insert(event).unwrap();
    }
    let counter = handler.start_count().unwrap();

    ptrace::cont(pid, None).unwrap();
    std::thread::spawn(move || loop {
        nix::sys::signal::kill(pid, nix::sys::signal::SIGSTOP).unwrap();
        std::thread::sleep(interval);
    });
    loop {
        let status =
            nix::sys::wait::waitpid(pid, Some(nix::sys::wait::WaitPidFlag::WSTOPPED)).unwrap();
        dbg!(status);

        let count = counter.read().unwrap();
        println!("TRUE: {:?}", count);

        ptrace::cont(pid, None).unwrap();
    }
}
