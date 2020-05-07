# {{project-name}}
Welcome to your Nspire project!

## Prerequisites
- Ndless toolchain installed and added to path
- Rustup installed
- Latest Rust Nightly installed (nightly-2020-05-07 works)
- Unix-like (tested on Linux, most likely Mac and Cygwin will work as well)

Complete install script:
```bash
curl https://sh.rustup.rs -sSf | sh # skip if rustup already installed
rustup install nightly # skip if nightly already installed
cargo install cargo-ndless
```

Get started by running

```bash
cargo +nightly ndless build
```

to start development. Your .tns file will be available in
`target/armv5te-nspire-eabi/debug/{{project-name}}.tns`.

When you're ready to release your application,
**don't forget to compile in release mode** with

```bash
cargo +nightly ndless build -- --release
```

Your .tns file will be available in
`target/armv5te-nspire-eabi/release/{{project-name}}.tns`.

If you have the Firebird emulator installed, you can also send the compiled
binary straight to it. Just run:

```bash
cargo +nightly ndless run
cargo +nightly ndless run -- --release
```

You may skip `+nightly` if you set nightly as your default compiler
(`rustup default nightly`), or
[set a directory override](https://github.com/rust-lang/rustup.rs#directory-overrides).

Check out the [ndless examples](https://github.com/lights0123/example-nspire)
for more info.
