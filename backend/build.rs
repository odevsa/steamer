use std::fs;
use std::path::Path;

fn main() {
    let frontend_build = Path::new("../frontend/build");
    if !frontend_build.exists() {
        let _ = fs::create_dir_all(frontend_build);
    }
    println!("cargo:rerun-if-changed=../frontend/build");
}
