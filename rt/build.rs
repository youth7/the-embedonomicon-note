// use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 构建脚本的输出目录

    println!("");
    // let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // // 扩展库的搜索路径
    // println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-search={}", "D:\\workspace\\rust\\app\\rt");
    
    // 将link.x复制到指定位置
    // File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    Ok(())
}