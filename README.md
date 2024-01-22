*** Environment setup: ***
```
$ rustup override set nightly

$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

$ rustup component add llvm-tools-preview

$ cargo install bootimage
```
*** Useful commands: ***

Build the image: 
```
$ cargo bootimage 
```

*** QEMU cmd for testing the image: ***
```
$ qemu-system-x86_64 -drive format=raw,file=bootimage-ardea-os.bin \
        -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio
```
