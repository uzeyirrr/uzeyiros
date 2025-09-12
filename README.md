# ÜzeyirOS - Personal Operating System

<div align="center">

![ÜzeyirOS Logo](https://img.shields.io/badge/ÜzeyirOS-v1.0.0-blue?style=for-the-badge&logo=rust)
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![x86_64](https://img.shields.io/badge/Architecture-x86_64-red?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**Personal Operating System Project by Üzeyir İsmail Bahtiyar**

</div>

## 🚀 About

ÜzeyirOS is a personalized version of RXV64, which is a Rust reimplementation of MIT's xv6 operating system. This project showcases my operating system development skills and serves as a learning portfolio demonstrating my journey in low-level systems programming.

### ✨ Features

- 🦀 **Written in Rust** - Modern, safe, and performant
- 🖥️ **x86_64 Architecture** - Multi-processor system support
- 📺 **CGA Display Mode** - Text-based interface
- ⌨️ **PS/2 Keyboard Support** - Full keyboard input
- 💾 **PCIe AHCI SATA** - Disk storage support
- 🔌 **Serial Port Support** - UART communication
- 🎨 **Customized Interface** - Personal welcome messages

## 🛠️ Technical Details

### Supported Hardware
- **CPU**: x86_64 multi-processor systems
- **Display**: CGA text mode (80x25)
- **Keyboard**: PS/2 keyboard controller
- **Storage**: PCIe AHCI SATA devices
- **Communication**: Serial port (UART)

### System Requirements
- **Rust**: Nightly toolchain
- **QEMU**: x86_64 emulator
- **LLVM**: objcopy utility
- **GCC**: C compiler

## 🚀 Installation and Running

### 1. Install Dependencies

```bash
# Rust nightly toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
rustup toolchain install nightly
rustup default nightly

# QEMU (Manjaro/Arch)
sudo pacman -S qemu-system-x86 base-devel

# VNC Viewer (for GUI)
sudo pacman -S tigervnc

# LLVM (for objcopy)
sudo pacman -S llvm
```

### 2. Build the Project

```bash
git clone <https://github.com/uzeyirrr/uzeyiros.git>
cd rxv64
cargo xtask build
```

### 3. Create Disk Image

```bash
gcc -o bin/mkfs bin/mkfs.c
./bin/mkfs sdahci0.img
```

### 4. Run the Operating System

```bash
# Build and run with VNC (recommended)
env -u CURSOR_AGENT -u RUSTUP_TOOLCHAIN PATH="$HOME/.cargo/bin:$PATH" cargo xtask run

# Or run QEMU directly with VNC
qemu-system-x86_64 -cpu qemu64,pdpe1gb,xsaveopt,fsgsbase,apic,msr -smp 8 -m 8192 -M q35 -device ahci,id=ahci0 -drive id=sdahci0,file=sdahci0.img,if=none,format=raw -device ide-hd,drive=sdahci0,bus=ahci0.0 -kernel target/x86_64-unknown-none-elf/release/rxv64.elf32 -vnc :0

# Connect with VNC viewer
vncviewer localhost:5900
```

**Note:** If you encounter Cursor proxy errors, use the environment variable workaround shown above.

## 🎮 Usage

When the operating system starts:
1. **Welcome message** will be displayed
2. **Keyboard input** is available
3. **Basic commands** can be used

### Troubleshooting

**Cursor Proxy Error:**
```bash
env -u CURSOR_AGENT -u RUSTUP_TOOLCHAIN PATH="$HOME/.cargo/bin:$PATH" cargo xtask run
```

**QEMU Display Issues:**
- Use VNC mode: `-vnc :0`
- Connect with: `vncviewer localhost:5900`
- Stop QEMU: `Ctrl+C` in terminal


## 🏗️ Project Structure

```
rxv64/
├── kernel/          # Operating system kernel (Rust)
├── cmd/             # User commands (C)
├── ulib/            # User libraries
├── syslib/          # System libraries
├── bin/             # Utility tools
└── xtask/           # Build system
```

## 🎯 Future Plans

- [ ] **Graphics interface** support
- [ ] **Network protocol** implementation
- [ ] **File system** improvements
- [ ] **Security** features
- [ ] **Performance** optimizations
- [ ] **New system calls**

## 👨‍💻 Developer



## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **MIT xv6** - Original operating system design
- **RXV64** - Rust implementation
- **Rust Community** - Amazing tools and documentation

---

<div align="center">

**⭐ If you liked this project, don't forget to give it a star!**

Made with ❤️ by Üzeyir İsmail Bahtiyar

</div>