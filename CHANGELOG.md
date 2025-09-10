# Changelog

All notable changes to the ÜzeyirOS project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Planned
- Network protocol implementation
- File system improvements
- Security features
- Advanced graphics support

## [1.1.0] - 2024-12-19

### ✨ Added
- **Interactive GUI Interface**: Beautiful graphical user interface with system information display
- **Real-time Shell**: Fully functional command-line shell with keyboard input support
- **Command System**: Complete command processing with help, echo, info, and clear commands
- **Dynamic Prompt**: Smart prompt positioning that moves down after each command output
- **Keyboard Input Handling**: PS/2 keyboard support with real-time character input
- **CGA Graphics**: Advanced CGA display functions including boxes, text positioning, and character placement

### 🔧 Technical Improvements
- Fixed build system compatibility issues with custom target specifications
- Improved multi-processor synchronization and CPU management
- Enhanced panic handling to prevent system deadlocks
- Optimized memory management and display buffer handling
- Better error handling and user feedback system

### 🎨 User Experience
- Professional-looking system information display
- Intuitive command-line interface
- Clear visual separation between GUI and shell areas
- Responsive keyboard input with backspace support
- Organized command output that doesn't overlap with GUI elements

## [1.0.0] - 2024-12-19

### ✨ Added
- Personal welcome message: "Hosgeldiniz! Ben Uzeyir Ismail Bahtiyar"
- Operating system startup message: "Uzeyiros Isletim Sistemi basariyla baslatildi!"
- Screen clearing functionality at startup
- Special message display for main CPU
- Personal portfolio README documentation
- CHANGELOG.md file

### 🔄 Changed
- Operating system name changed to "ÜzeyirOS"
- Startup messages translated to Turkish
- README.md converted to personal portfolio format
- Project description focused on personal development

### 🐛 Fixed
- `target-pointer-width` format error in `lib/x86_64-unknown-none-elf.json`
- `target-pointer-width` format error in `lib/x86_64-unknown-rxv64-elf.json`
- Build system Rust nightly toolchain compatibility

### 🔧 Technical
- JSON format fixes in target files
- Build system improvements
- Code organization optimizations

## [0.1.0] - 2024-12-19

### ✨ Added
- RXV64 basic implementation
- x86_64 multi-processor system support
- CGA display mode support
- PS/2 keyboard controller
- PCIe AHCI SATA storage support
- Serial port (UART) support
- Basic file system
- User programs (ls, cat, echo, grep, wc, etc.)
- Build system (xtask)
- QEMU emulator support

### 🏗️ Architecture
- Kernel written in Rust
- User programs written in C
- x86_64 assembly code segments
- Modular design

### 📁 Project Structure
- `kernel/` - Operating system kernel
- `cmd/` - User commands
- `ulib/` - User libraries
- `syslib/` - System libraries
- `bin/` - Utility tools
- `xtask/` - Build system

---

## Version Numbering

This project uses [Semantic Versioning](https://semver.org/):
- **MAJOR**: Incompatible API changes
- **MINOR**: Backward compatible new features
- **PATCH**: Backward compatible bug fixes

## Change Types

- **Added**: New features
- **Changed**: Changes to existing features
- **Deprecated**: Features that will be removed soon
- **Removed**: Features removed in this version
- **Fixed**: Bug fixes
- **Security**: Security vulnerability fixes