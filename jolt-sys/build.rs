use std::env;
use std::path::PathBuf;

use walkdir::WalkDir;

fn main() {
    build();
    link();
    generate_bindings();

    println!("cargo:rerun-if-changed=JoltC/JoltPhysicsC.h");
}

fn build() {
    build_jolt();
    build_joltc();
}

fn build_joltc() {
    let mut build = cc::Build::new();

    for entry in WalkDir::new("JoltC") {
        let entry = entry.unwrap();
        let file_name = entry
            .file_name()
            .to_str()
            .expect("file was not valid UTF-8");

        if file_name.ends_with(".cpp") {
            build.file(entry.path());
        }
    }

    build
        .std("c++17")
        .include(".")
        .include("JoltC")
        .cpp(true)
        .compile("JoltC");
}

fn build_jolt() {
    let mut build = cc::Build::new();

    for entry in WalkDir::new("Jolt") {
        let entry = entry.unwrap();
        let file_name = entry
            .file_name()
            .to_str()
            .expect("file was not valid UTF-8");

        if file_name.ends_with(".cpp") {
            build.file(entry.path());
        }
    }

    build.std("c++17").include(".").cpp(true).compile("Jolt");
}

fn link() {
    println!("cargo:rustc-link-lib=Jolt");
    println!("cargo:rustc-link-lib=JoltC");
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("JoltC/JoltPhysicsC.h")
        .allowlist_item("JPC_+.*")
        .default_enum_style(bindgen::EnumVariation::Consts)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
