use std::env;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    #[cfg(windows)]
    #[cfg(target_pointer_width = "64")]
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("lib").join("win").join("64").display()
    );

    #[cfg(windows)]
    #[cfg(target_pointer_width = "32")]
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new(&dir).join("lib").join("win").join("32").display()
    );
}
