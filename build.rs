use anyhow::Context;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    build_simavr(&out_path);
    build_simavr_bindings(&out_path);
    link_libelf();
}

fn build_simavr(out: &Path) {
    println!("=> Building simavr");

    if !Path::new("vendor")
        .join("simavr")
        .join("README.md")
        .exists()
    {
        panic!("\
            `vendor/simavr` doesn't seem to contain the expected source code. \
            If you're cloning simavr-ffi by hand, please use `git clone ... --recurse-submodules`"
        );
    }

    #[cfg(target_family = "unix")]
    build_simavr_unix(out);

    #[cfg(not(target_family = "unix"))]
    panic!("Sorry, I don't know how to build simavr on this architecture");
}

#[cfg(target_family = "unix")]
fn build_simavr_unix(out: &Path) {
    let out_simavr = out.join("simavr");

    if !out_simavr.exists() {
        fs::create_dir(&out_simavr)
            .with_context(|| format!("Couldn't create directory: {}", out_simavr.display()))
            .unwrap();
    }

    let result = Command::new("make")
        .current_dir("vendor/simavr/simavr")
        .env("OBJ", out_simavr.as_os_str())
        .arg("-e")
        .arg("libsimavr")
        .status()
        .expect("Couldn't build simavr");

    if !result.success() {
        panic!("Couldn't build simavr: `make` failed");
    }

    println!("cargo:rustc-link-search={}", out_simavr.display());
    println!("cargo:rustc-link-lib=static={}", "simavr");
}

fn build_simavr_bindings(out: &Path) {
    println!("=> Building simavr bindings");

    let bindings = bindgen::Builder::default()
        .header("vendor/simavr/simavr/sim/avr_ioport.h")
        .header("vendor/simavr/simavr/sim/avr_uart.h")
        .header("vendor/simavr/simavr/sim/sim_avr.h")
        .header("vendor/simavr/simavr/sim/sim_elf.h")
        .generate()
        .expect("Couldn't generate simavr's bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write simavr's bindings");
}

fn link_libelf() {
    println!("=> Linking libelf");

    pkg_config::probe_library("libelf").unwrap();

    println!("cargo:rustc-link-lib=static=elf");
}
