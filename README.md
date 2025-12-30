# Rust Shellcode Dropper

A minimal shellcode dropper written in Rust using raw FFI bindings to kernel32.dll.

## Build

```bash
# On Windows
cargo build --release

# Cross-compile from Linux/macOS
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Run

```bash
.\target\release\shellcode_dropper.exe
```

## ⚠️ Disclaimer

This project is provided **strictly for educational and research purposes only**.

By using this software, you agree to use it **ethically and legally**. The developer assumes no responsibility or liability for any misuse, damage, or illegal activity resulting from the use of this code.

**Do not use this tool for unauthorized access to systems.** Always obtain proper authorization before testing on any system you do not own.

