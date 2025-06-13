use anyhow::Context;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    check();
    copy(&out);
    build(&out);
    generate_bindings(&out);
    link();
}

fn check() {
    if !Path::new("vendor")
        .join("simavr")
        .join("README.md")
        .exists()
    {
        panic!(
            "`vendor/simavr` doesn't exist - if you're cloning simavr-ffi by \
             hand, please use `git clone ... --recurse-submodules`"
        );
    }
}

fn copy(out: &Path) {
    println!("=> Copying simavr");

    let out_simavr = out.join("simavr");

    if out_simavr.exists() {
        fs::remove_dir_all(&out_simavr)
            .with_context(|| format!("Couldn't remove directory: {}", out_simavr.display()))
            .unwrap();
    }

    fs::create_dir(&out_simavr)
        .with_context(|| format!("Couldn't create directory: {}", out_simavr.display()))
        .unwrap();

    fs_extra::copy_items(&["vendor/simavr"], &out, &Default::default())
        .with_context(|| {
            format!(
                "Couldn't copy simavr's sources to: {}",
                out_simavr.display()
            )
        })
        .unwrap();
}

fn build(out: &Path) {
    println!("=> Building simavr");

    #[cfg(target_family = "unix")]
    build_unix(out);

    #[cfg(not(target_family = "unix"))]
    panic!("simavr-ffi can be built only on Unixes right now - pull requests are welcome!");
}

#[cfg(target_family = "unix")]
fn build_unix(out: &Path) {
    let out_simavr = out.join("simavr");

    let result = Command::new("make")
        .current_dir(out_simavr.join("simavr"))
        .env("OBJ", out_simavr.as_os_str())
        .arg("-e")
        .arg("libsimavr")
        .status()
        .expect("Couldn't build simavr");

    if !result.success() {
        panic!("Couldn't build simavr: `make` failed");
    }

    println!("cargo:rustc-link-search={}", out_simavr.display());
    println!("cargo:rustc-link-lib=static=simavr");
}

fn generate_bindings(out: &Path) {
    println!("=> Generating simavr bindings");

    let simavr = out.join("simavr").join("simavr").join("sim");
    let mut builder = bindgen::Builder::default();

    let headers = WalkDir::new(simavr)
        .max_depth(1)
        .into_iter()
        .map(|header| header.unwrap())
        .filter(|entry| {
            let metadata = entry.metadata().unwrap();
            let name = entry.file_name().to_string_lossy();

            metadata.is_file()
                && name.ends_with(".h")
                && name != "sim_cmds.h"
                && name != "sim_core.h"
        })
        .map(|header| header.path().to_string_lossy().to_string());

    for header in headers {
        builder = builder.header(header);
    }

    let bindings = builder
        .generate()
        .expect("Couldn't generate simavr's bindings");

    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write simavr's bindings");
}

fn link() {
    link_libelf();
    link_zlib();
    link_libzstd();
}

fn link_libelf() {
    println!("=> Linking libelf");

    pkg_config::probe_library("libelf").unwrap();

    println!("cargo:rustc-link-lib=static=elf");
}

fn link_zlib() {
    println!("=> Linking zlib");

    pkg_config::probe_library("zlib").unwrap();

    println!("cargo:rustc-link-lib=z");
}

fn link_libzstd() {
    println!("=> Linking libzstd");

    pkg_config::probe_library("libzstd").unwrap();

    println!("cargo-rustc-link-lib=zstd");
}
