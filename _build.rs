use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=jigsaw");
    println!(
        "cargo:rustc-link-search=native={}",
        out_path.canonicalize().unwrap().display()
    );
}
