# rust + os = rost

An operating system in Rust.

rost is heavily inspired by and based on [blog_os](https://github.com/phil-opp/blog_os),
but sometimes modified.
See the licenses from [blog_os](https://github.com/phil-opp/blog_os) [LICENSE_APACHE](blog_os/LICENSE_APACHE)
and [LICENSE_MIT](blog_os/LICENSE_MIT)

# Dependencies

Install rust source code. Needed for compiling `core` for rost's target triple.

```
rustup component add rust-src
```

Install `bootimage` tool.

```
cargo install bootimage
rustup component add llvm-tools-preview
```

Install `qemu` as virtual machine.

```
sudo apt-get install qemu-system-x86
```

