//! 链接脚本注入：am/link/linker.lds 用绝对路径传给链接器，
//! 避免相对 `-T` 路径因链接器工作目录而失效。

use std::path::PathBuf;

fn main() {
    let ld = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../am/link/linker.lds")
        .canonicalize()
        .expect("找不到 am 链接脚本 am/link/linker.lds");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rerun-if-changed={}", ld.display());
}
