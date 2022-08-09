在Win10下运行以下命令进行编译

```bash
cargo rustc --target thumbv7m-none-eabi -- --emit=obj
```

再运行以下命令查看目标文件中符号

```bash
rust-nm  target/thumbv7m-none-eabi/debug/deps/app-*.o
```

查看符号也可以在WSL下使用其它工具进行，例如使用`readelf`
```bash
readelf -s ./target/thumbv7m-none-eabi/debug/deps/*.o
```

或者使用`nm`
```bash
nm ./target/thumbv7m-none-eabi/debug/deps/*.o
```

