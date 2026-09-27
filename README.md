[English](README.md) | [Русский](README_RU.md)

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
Arch (PC) <img width="954" height="1037" alt="изображение" src="https://github.com/user-attachments/assets/33568879-8d47-4d50-a828-511879f0280d" />
Arch (LapTop) <img width="1142" height="801" alt="image" src="https://github.com/user-attachments/assets/b7e8165b-32a3-460b-8bb1-4bdfacf3f765" />
Artix <img width="1254" height="759" alt="image" src="https://github.com/user-attachments/assets/1ce05a92-367c-4d50-a314-662a2cc913df" />
NixOS  <img width="1201" height="766" alt="изображение" src="https://github.com/user-attachments/assets/a37252bb-5167-48c0-a14a-fe7d790f050a" />

> 🐧 **Supported Logos:** AlmaLinux, Alpine, Alt Linux, Android, Apple (macOS), Arch, Artix, Asahi, Astra Linux, CachyOS, CentOS, ChromeOS, Debian, Deepin, Elementary OS, EndeavourOS, Fedora, FreeBSD, Garuda, Gentoo, Kali Linux, KDE neon, Kubuntu, Lubuntu, Manjaro, Linux Mint, MX Linux, NetBSD, NixOS, OpenBSD, openSUSE, Parrot OS, Pop!_OS, Proxmox, Raspberry Pi OS, RHEL, Rocky Linux, Slackware, Solus, SteamOS, Tails, TrueNAS, Tux, Ubuntu, Void Linux, Windows, Xubuntu, Zorin OS. More coming soon!  
> *If your distro isn't explicitly supported yet, SysPrint will fall back to the standard GNU/Linux penguin Tux logo.*

---
### Config

* **Linux / BSD:** `~/.config/sysprint/config.toml`
* **Windows:** `%APPDATA%\sysprint\config.toml` *(usually `C:\Users\Имя\AppData\Roaming\sysprint\config.toml`)*

### Example `config.toml`

```toml
# SysPrint Configuration

# Custom logo override by name (e.g. "arch", "debian", "ubuntu", "fedora", "windows", "tux", "apple", "gentoo", etc.)
# Leave empty for automatic OS detection.
logo = ""

# Modes & behavior
mini-logo-mode = false
fast-mode = false
compact-mode = false
config-stronger = false

# Master section toggles
show-system-info = true
show-cpu-info = true
show-gpu-info = true
show-memory-info = true
show-other-info = true
show-disks-info = true

# Granular: System
show-os = true
show-kernel = true
show-os-version = true
show-init = true
show-host = true
show-user = true
show-uptime = true
show-processes = true

# Granular: CPU
show-cpu-name = true
show-cpu-freq = true
show-cpu-usage = true
show-cpu-temp = true
show-cpu-cores = true
show-cpu-arch = true

# Granular: GPU
show-gpu-name = true
show-gpu-temp = true
show-gpu-vram = true

# Granular: Memory
show-ram = true
show-swap = true

# Granular: Other
show-de = true
show-wm = true
show-terminal = true
show-shell = true
show-local-ip = true
show-battery = true
show-locale-time = true
show-fetch-info = true

# Granular: Disks
show-disks = true
```

## CLI Arguments

| Flag / Option | Description |
|---|---|
| `--logo <NAME>` | Override ASCII logo with a specific system logo (e.g. `debian`, `arch`, `ubuntu`, `fedora`, `windows`, `apple`, `tux`, etc.) |
| `--mini` | Display a small 5-line mini ASCII logo |
| `--fast-mode` | Skip heavy telemetry (disks, network interfaces, multi-pass CPU usage) for ultra-fast startup |
| `--compact-mode` | Show concise output without decorative headers and deep details |
| `--generate-config` | Generate a new default `config.toml` in the user's config directory |
| `--no-pause` | Windows: do not wait for Enter keypress before exiting |
| `--hide-system` | Hide the System Information section |
| `--hide-cpu` | Hide the CPU section |
| `--hide-gpu` | Hide the GPU section |
| `--hide-memory` | Hide the Memory / Swap section |
| `--hide-other` | Hide the Other (Desktop, Shell, IP, Battery) section |
| `--hide-disks` | Hide the Disks section |
| `--hide-fetch-info` | Hide the final "SysPrint vX.Y.Z" footer line |

## Features
- 🚀 Blazing fast performance thanks to Rust
- 🎨 Beautiful ASCII art logos and colored CLI output
- 💻 Displays CPU, RAM, Swap, GPU, OS, Processes, IP, Disks and more using the `sysinfo` crate

---

## Installation

### Linux

#### Option 1: Fast Install (Precompiled Binary)
Download the latest binary from the [Releases page](https://github.com/MBKCHEL/SysPrint/releases/latest) and install to download folder:
```bash
chmod +x ~/Downloads/sysprint-linux
sudo mv ~/Downloads/sysprint-linux /usr/local/bin/sysprint
```
To run:
```bash
sysprint
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
# For Bash
echo "sysprint" >> ~/.bashrc

# For Zsh
echo "sysprint" >> ~/.zshrc

# For Fish
echo "sysprint" >> ~/.config/fish/config.fish
```
### FreeBSD / OpenBSD / NetBSD
Option 1: Fast Install (Precompiled Binary)
Download the sysprint-freebsd binary from the Releases page
``` bash
chmod +x ~/Downloads/sysprint-freebsd
sudo mv ~/Downloads/sysprint-freebsd /usr/local/bin/sysprint
```
To run:
``` bash
sysprint
```

### Windows
Download the `.exe` from the Releases page and run it.

## Uninstallation
## Linux & BSD(FreeBSD, OpenBSD, NetBSD)
1. Remove binary:
   ``` bash
   sudo rm /usr/local/bin/sysprint
   ```
2. Remove from terminal auto-run (if added):
   ``` bash
   # Bash
   sed -i '/sysprint/d' ~/.bashrc

   # Zsh
   sed -i '/sysprint/d' ~/.zshrc

   # Fish
   fish -c "sed -i '/sysprint/d' ~/.config/fish/config.fish"
   ```


## Windows 
1. Delete the downloaded sysprint.exe file.
2. If added to startup, press Win + R, type shell:startup, and delete the sysprint shortcut.
    
