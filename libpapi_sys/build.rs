extern crate bindgen;
extern crate num_cpus;

use std::path::PathBuf;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let original_cflags = std::env::var("CFLAGS").unwrap_or_default();
    let new_cflags = format!("{} -fPIC", original_cflags);
    std::env::set_var("CFLAGS", new_cflags);


    let out_dir: String = std::env::var("OUT_DIR").unwrap();
    let target_pipe_source_dir: PathBuf = PathBuf::from(format!("{}/libpapi", out_dir));

    let mut papi_source_dir = std::env::current_dir()?;
    papi_source_dir.push("libpapi");
    papi_source_dir.push("src");

    let mut mkdir = Command::new("mkdir")
        .args(&["-p", &format!("{}", target_pipe_source_dir.display())])
        .spawn()?;
    mkdir.wait()?;

    let mut copy = Command::new("cp")
        .args(&[
            "-r",
            &format!("{}", papi_source_dir.display()),
            &format!("{}", target_pipe_source_dir.display()),
        ])
        .spawn()?;
    let target_pipe_source_dir: PathBuf = PathBuf::from(format!("{}/libpapi/src", out_dir));
    copy.wait()?;

    println!("INFO: start ./configure");
    let mut configure = Command::new("./configure")
        .current_dir(&target_pipe_source_dir)
        .spawn()?;
    match configure.wait() {
        Ok(_) => {},
        Err(_) => {
            println!("WARNING: configure error")
        },
    };
    println!("INFO: ./configure finished");

    let cpu_num = num_cpus::get();

    println!("INFO: start make");
    let mut make = Command::new("make")
        .args(&[format!("-j{}", cpu_num), "--keep-going".to_owned()])
        .current_dir(&target_pipe_source_dir)
        .spawn()?;
    match make.wait() {
        Ok(_) => {},
        Err(_) => {
            println!("WARNING: make error")
        },
    };
    println!("INFO: make finished");

    let file_path = match std::env::var("TARGET").unwrap_or("".to_owned()).as_str() {
        "x86_64-unknown-linux-gnu" => PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("bindings")
            .join("bindings.rs"),
        _ => {
            let bindings = bindgen::Builder::default()
                .header("./libpapi/src/papi.h")
                .generate()
                .expect("Unable to generate bindings");
            let out_path = PathBuf::from(out_dir).join("bindings.rs");
            bindings
                .write_to_file(out_path.clone())
                .expect("Couldn't write bindings!");

            out_path
        }
    };
    println!(
        "cargo:rustc-env=BINDING_PATH={}",
        file_path.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=static=papi");
    println!(
        "cargo:rustc-link-search=native={}",
        target_pipe_source_dir.display()
    );
    Ok(())
}
