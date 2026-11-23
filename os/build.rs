use std::path::PathBuf;

fn main() {
    let ld = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../am/link/linker.lds")
        .canonicalize()
        .expect("找不到 am 链接脚本 am/link/linker.lds");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rerun-if-changed={}", ld.display());
}
