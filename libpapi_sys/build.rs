extern crate bindgen;
extern crate num_cpus;

use std::process::Command;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let mut current_dir = std::env::current_dir()?;
    current_dir.push("libpapi");
    current_dir.push("src");

    let mut configure = Command::new("./configure")
        .current_dir(&current_dir)
        .spawn()?;
    configure.wait()?;

    let cpu_num = num_cpus::get();
    let mut make = Command::new("make")
        .args(&[format!("-j{}", cpu_num)])
        .current_dir(&current_dir)
        .spawn()?;
    make.wait()?;

    let bindings = bindgen::Builder::default()
        .header("./libpapi/src/papi.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=static=papi");
    println!("cargo:rustc-link-search=native={}", current_dir.into_boxed_path().display());
    Ok(())
}
