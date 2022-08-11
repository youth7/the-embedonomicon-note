在Win10下运行以下命令进行编译
```Rust
cargo build --bin app
```

成功后使用以下命令来检查结果是否符合预期

```powershell
# 使用rust-objdump去检查最终生成的可执行文件中指定节的具体内容
rust-objdump -s --section .vector_table .\target\thumbv7m-none-eabi\debug\app
```


接着启动一个Qemu模拟器用来debug程序
```powershell
qemu-system-arm `
      -cpu cortex-m3 `
      -machine lm3s6965evb `
      -gdb tcp::3333 `
      -S `
      -nographic `
      -kernel target/thumbv7m-none-eabi/debug/app
```


在另外一个窗口启动GDB进行远程调试
```powershell
arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/app
Reading symbols from target/thumbv7m-none-eabi/debug/app...
(gdb) target remote :3333 	#连接远程调试服务器，程序会自动停止在第1行等待调试
Remote debugging using :3333
app::Reset () at src/main.rs:12
12      pub unsafe extern "C" fn Reset() -> ! {
(gdb) print/x $sp			#打印栈顶的值
$1 = 0x20010000
(gdb) s						#运行下一行代码
13          let _x = 42;
(gdb) s						#运行下一行代码
15          loop {}
(gdb) print _x				#打印变量_x的值
$2 = 42
(gdb) print &_x				#打印变量_x的地址
$3 = (*mut i32) 0x2000fffc
(gdb) quit					#退出
```

