Robot on Rust
=============

This is an attempt to code an FRC robot in Rust, using the `wpilib` crate.

Compiling
---------
1. Install the [FRC RoboRIO toolchain](https://github.com/wpilibsuite/roborio-toolchain/releases)
2. Create .cargo/config with the following contents:
```toml
[target.arm-unknown-linux-gnueabi]
linker = # path to toolchain's arm-frc2022-linux-gnueabi-gcc executable
rustflags = [
  "-C", "target-cpu=cortex-a9",
]

[build]
target = "arm-unknown-linux-gnueabi"
```
3. Add the necessary Rust target with `rustup target add arm-unknown-linux-gnueabi`
4. Build the project with `cargo build`

Note: the wpilib-sys crate has a broken build script on Windows.  
It should work fine on UNIX-like systems, but `fs::remove_file` does not delete folder symlinks on Windows, causing "file already exists" errors on all but the first build.  
Adding an `fs::remove_dir` fixes the issue.  
TODO: PR a fix upstream

wpilibfill
----------
Much has been added to the `wpilib` crate since the latest release in 2019. Unfortuantely, an attempt to use the latest master as a dependency instead of `0.4.0` led to compile errors in `wpilib-sys`. Instead of properly fixing the issue, the newer parts of `wpilib` that are used in this crate were copied into the `wpilibfill` module and modified to work with `wpilib-sys = 0.4.0`. This is a bit of an ugly workaround used to at least have something that compiles.  
TODO: Fix this issue more properly and remove `wpilibfill`
