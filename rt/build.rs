use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    //从环境变量OUT_DIR中读取一个路径，用于存放构建过程的一些中间产物
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    //这是最重要的一步了，通过特定的指令告诉编译器从哪个路径搜索链接脚本
    println!("cargo:rustc-link-search={}", out_dir.display());

    // 将链接脚本复制到上一步指定的路径
    // 如果在上一步中将链接脚本的搜索路径设置为库的根目录，则这一步可以省略
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;
    
    Ok(())
}