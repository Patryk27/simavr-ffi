#![feature(exit_status_error)]

use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // ---

    eprintln!("=> Compiling simavr");

    Command::new("make")
        .current_dir("vendor/simavr/simavr")
        .status()
        .expect("Couldn't build simavr")
        .exit_ok()
        .expect("Couldn't build simavr");

    println!(
        "cargo:rustc-link-search={}",
        "vendor/simavr/simavr/obj-x86_64-unknown-linux-gnu" // TODO
    );

    println!("cargo:rustc-link-lib=static={}", "simavr");

    // ---

    eprintln!("=> Generating bindings");

    let bindings = bindgen::Builder::default()
        .header("vendor/simavr/simavr/sim/avr_ioport.h")
        .header("vendor/simavr/simavr/sim/avr_uart.h")
        .header("vendor/simavr/simavr/sim/sim_avr.h")
        .header("vendor/simavr/simavr/sim/sim_elf.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    // ---

    eprintln!("=> Linking libelf");

    pkg_config::probe_library("libelf").unwrap();

    println!("cargo:rustc-link-lib=static=elf");
}
