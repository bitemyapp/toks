use std::{env, fs};
fn main() {
    let isa = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        a => panic!("unsupported architecture: {a}"),
    };
    let dir = format!("../src/asm/{isa}");
    let mut asm = cc::Build::new();
    asm.include("../src/core").include("../include").include(&dir);
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|x| x == "S") { asm.file(path); }
    }
    asm.compile("toks_asm");
    println!("cargo:rerun-if-changed=../src/asm");
    println!("cargo:rerun-if-changed=../src/core/layout.h");
}
