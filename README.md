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

> 🐧 **Supported Logos:** Alpine, Android, MacOS, Arch, Artix, Astra Linux, EndeavourOS, CachyOS, Debian, Fedora, FreeBSD,NetBSD, OpenBSD, OpenSUSE,Gentoo, Kali Linux, Manjaro, Linux Mint, NixOS, Pop!_OS, Ubuntu. Void Linux, Zorin OS, and Windows. More coming soon!  
> *If your distro isn't explicitly supported yet, SysPrint will fall back to the standard GNU/Linux penguin Tux logo.*

---
### Config

* **Linux / BSD:** `~/.config/sysprint/config.toml`
* **Windows:** `%APPDATA%\sysprint\config.toml` *(usually `C:\Users\Имя\AppData\Roaming\sysprint\config.toml`)*

### Example `.sysprint.toml`

```toml
# SysPrint configuration
show-system-info = true
show-cpu-info = true
show-memory-info = true
show-disks-info = true
show-other-info = true
show-gpu-info = true
mini-mode = false
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
Download the latest binary from the [releases page](https://github.com/MBKCHEL/SysPrint/releases/latest) and install to download folder:
```bash
chmod +x ~/Downloads/sysprint-linux
sudo mv ~/Downloads/sysprint-linux /usr/local/bin/sysprint
```
To run:
``` bash
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

##### Nix / NixOS
For Nix or NixOS, you can install the program as follows:

1. Quick start
```bash
nix run github:MBKCHEL/SysPrint
```

2. Through flake
```nix
inputs = {
  nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  sysprint = {
    url = github:MBKCHEL/SysPrint;
  };
};
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
    
