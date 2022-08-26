使用以下命令在WIN10下编译：
```powershell
rustup default nightly # 切换为nightly
rustup target add thumbv7m-none-eabi #在nightly下需要先重新安装target
cargo  build --bin app# 编译项目
```