extern crate bindgen;
extern crate num_cpus;

use std::process::Command;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let out_dir:String = std::env::var("OUT_DIR").unwrap();
    let target_pipe_source_dir:PathBuf = PathBuf::from(format!("{}/libpapi", out_dir));

    let mut papi_source_dir = std::env::current_dir()?;
    papi_source_dir.push("libpapi");
    papi_source_dir.push("src");

    let mut mkdir = Command::new("mkdir")
        .args(&["-p", &format!("{}", target_pipe_source_dir.display())])
        .spawn()?;
    mkdir.wait()?;

    let mut copy = Command::new("cp")
        .args(&["-r", &format!("{}", papi_source_dir.display()), &format!("{}", target_pipe_source_dir.display())])
        .spawn()?;
    let target_pipe_source_dir:PathBuf = PathBuf::from(format!("{}/libpapi/src", out_dir));
    copy.wait()?;

    let mut configure = Command::new("./configure")
        .current_dir(&target_pipe_source_dir)
        .spawn()?;
    configure.wait()?;

    let cpu_num = num_cpus::get();
    let original_cflags = std::env::var("CFLAGS").unwrap_or_default();
    let mut make = Command::new("make")
        .args(&[format!("-j{}", cpu_num)])
        .env("CFLAGS", format!("{} -fPIC", original_cflags))
        .current_dir(&target_pipe_source_dir)
        .spawn()?;
    make.wait()?;

    let bindings = bindgen::Builder::default()
        .header("./libpapi/src/papi.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=static=papi");
    println!("cargo:rustc-link-search=native={}", target_pipe_source_dir.display());
    Ok(())
}
