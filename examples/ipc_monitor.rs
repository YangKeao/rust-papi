#[macro_use]
extern crate clap;

use papi::{Papi, EventSet, Event};
use clap::{App, Arg};
use std::convert::TryFrom;

use nix::unistd::Pid;
use nix::sys::ptrace;
use libc::pid_t;

fn main() {
    let matches = App::new("Counter Monitor")
        .version("0.1")
        .author("Yang Keao <keao.yang@yahoo.com>")
        .arg(Arg::with_name("tid")
            .short("t")
            .long("tid")
            .value_name("tid")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("interval")
            .short("i")
            .long("interval")
            .value_name("interval")
            .required(false)
            .takes_value(true))
        .get_matches();

    let tid = value_t!(matches, "tid", u64).unwrap();
    let interval = value_t!(matches, "interval", u64).unwrap_or(1000);
    let interval = std::time::Duration::from_millis(interval);

    let instance = Papi::new().unwrap();

    let event_set = EventSet::new().unwrap();
    event_set.assign_component(0).unwrap();

    let pid = Pid::from_raw(tid as pid_t);
    ptrace::attach(pid.clone()).unwrap();
    nix::sys::wait::waitpid(pid.clone(), Some(nix::sys::wait::WaitPidFlag::WSTOPPED)).unwrap();

    let handler = instance.attach(&event_set, tid).unwrap();

    let event = Event::try_from("PAPI_TOT_INS").unwrap();
    event_set.insert(event).unwrap();
    let event = Event::try_from("PAPI_TOT_CYC").unwrap();
    event_set.insert(event).unwrap();

    let counter = handler.start_count().unwrap();

    ptrace::cont(pid, None).unwrap();
    std::thread::spawn(move || {
        loop {
            nix::sys::signal::kill(pid.clone(), nix::sys::signal::SIGSTOP).unwrap();
            std::thread::sleep(interval);
        }
    });
    loop {
        let status = nix::sys::wait::waitpid(pid.clone(), Some(nix::sys::wait::WaitPidFlag::WSTOPPED)).unwrap();
        dbg!(status);

        let count = counter.read_and_reset().unwrap();
        println!("{:?}", (count[0] as f64 ) /( count[1] as f64));

        ptrace::cont(pid, None).unwrap();
    }
}