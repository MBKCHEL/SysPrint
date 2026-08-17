# SysPrint

A fast, lightweight, and customizable system information fetch tool written in Rust. Inspired by `neofetch` and `fastfetch`.

---
## Contributors

* [@dev-er1](https://github.com/dev-er1) - my first contributors, respect 
* [@MBKCHEL](https://github.com/MBKCHEL) - it`s me (owner)
* [@BALBES](https://github.com/BALB3S) - QA, and my best friend)))
---
## 🖼️ Preview & Screenshots

Windows <img width="972" height="494" alt="изображение" src="https://github.com/user-attachments/assets/b266bc19-46ad-45bc-b08b-61df368fc4f8" />
Debian <img width="1005" height="570" alt="изображение" src="https://github.com/user-attachments/assets/95c5627f-4b99-4a92-9fbd-28b8b025d149" />
Mint <img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/f748baf5-4be8-4286-b759-280d0ad794e5" />
Arch <img width="1280" height="719" alt="изображение" src="https://github.com/user-attachments/assets/29320bfc-1e16-44af-adb0-9524537f49b3" />

> 🐧 **Supported Logos:** Arch, Debian, Linux Mint, Ubuntu, macOS, Gentoo, Fedora, Kali, Manjaro, void Linux, NixOS, Pop!_OS, ZorinOS, OpenBSD, FreeBSD, NetBSD, and Windows. More coming soon!  
> *If your distro isn't explicitly supported yet, SysPrint will fall back to the standard GNU/Linux penguin Tux logo.*

---
### Config

* **Linux / BSD:** `~/.config/.sysinfo.toml`
* **Windows:** `%APPDATA%\.sysinfo.toml` *(usually `C:\Users\<Username>\AppData\Roaming\.sysinfo.toml`)*

### Example `.sysinfo.toml`

```toml
# SysPrint configuration
show-system-info = true
show-cpu-info = true
show-memory-info = true
show-disks-info = true
show-other-info = true
show-gpu-info = true
```
## When a CLI flag contradicts the config, `config-stronger = true` makes the config win
config-stronger = false



## Features
- 🚀 Blazing fast performance thanks to Rust
- 🎨 Beautiful ASCII art logos and colored CLI output
- 💻 Displays CPU, RAM, OS, Kernel, info using the `sysinfo` crate

---

## Installation

### Linux

#### Option 1: Fast Install (Precompiled Binary)
Download the latest binary from the (https://github.com/MBKCHEL/SysPrint/releases/tag/3.0.1) and install to download folder:
```bash
chmod +x ~/Downloads/sysprint-linux
sudo mv ~/Downloads/sysprint-linux /usr/local/bin/sysprint
```
To run:
```
sysprint
```

*Или Если у вас русский интерфейс:*
```bash
chmod +x ~/Загрузки/sysprint-linux
sudo mv ~/Загрузки/sysprint-linux /usr/local/bin/sysprint
```

#### Option 2: Build from Source
```bash
git clone https://github.com/MBKCHEL/SysPrint.git
cd SysPrint
cargo build --release
sudo cp target/release/sysprint /usr/local/bin/
```

#### Auto-run on Terminal Startup (Optional)
```bash
echo "sysprint" >> ~/.bashrc
```
### FreeBSD / OpenBSD / NetBSD
Option 1: Fast Install (Precompiled Binary)
Download the sysprint-freebsd binary from the Releases page
```
chmod +x ~/Downloads/sysprint-freebsd
sudo mv ~/Downloads/sysprint-freebsd /usr/local/bin/sysprint
```
To run:
```
sysprint
```

### Windows
Download the `.exe` from the Releases page and run it.

## Uninstallation
## Linux & BSD(FreeBSD, OpenBSD, NetBSD)
1. Remove binary:
   ```
   sudo rm /usr/local/bin/sysprint
   ```
2. Remove from terminal auto-run (if added):
   ```
   sed -i '/sysprint/d' ~/.bashrc
   ```
   (If using zsh, replace ~/.bashrc with ~/.zshrc)


## Windows 
1. Delete the downloaded sysprint.exe file.
2. If added to startup, press Win + R, type shell:startup, and delete the sysprint shortcut.
    
