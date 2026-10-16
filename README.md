# SysPrint

A fast, lightweight, and customizable system information fetch tool written in Rust. Inspired by `neofetch` and `fastfetch`.

---
## 🎨 Logo Gallery & Customization

Check out [LOGOS.md](./LOGOS.md) to see all supported OS logos, mini-modes, and custom ASCII examples.

> 🐧 **Supported Logos:** Alpine, Android, MacOS, Arch, Artix, Astra Linux, EndeavourOS, CachyOS, Debian, Fedora, FreeBSD,NetBSD, OpenBSD, OpenSUSE,Gentoo, Kali Linux, Manjaro, Linux Mint, NixOS, Pop!OS, Ubuntu, Void Linux, Zorin OS, Lubuntu, Xubuntu, Kubuntu, and Windows. More coming soon!  
> *If your distro isn't explicitly supported yet, SysPrint will fall back to the standard GNU/Linux penguin Tux logo.*

---
### Config

* **Linux / BSD:** `~/.config/sysprint/config.toml`
* **Windows:** `%APPDATA%\sysprint\config.toml` *(usually `C:\Users\Имя\AppData\Roaming\sysprint\config.toml`)*

### Example `.sysprint.toml`

```toml
# SysPrint configuration
# When a CLI flag contradicts the config, `config-stronger = true` makes the config win.
# Fast-mode disable GPU Vram and Temp info.

logo = "default"
logo-color = "purple"
custom-logo-path = "~/FemboyLogo.txt"
config-stronger = false
mini-logo-mode = false
fast-mode = false
compact-mode = false
hide-fetch-info = false
show-sysprint-start-time = true

[system]
enabled = true
os-name = true
kernel = true
os-version = true
init = true
host = true
user = true
uptime = true

[cpu]
enabled = true
name = true
ghz = true
usage = true
temp = true
cores = true
threads = true
architecture = true

[gpu]
enabled = true
name = true
vram = true
temp = true

[memory]
enabled = true
ram = true

[other]
enabled = true
de = true
wm = true
terminal = true
shell = true
battery = true
locale-time = true

[disks]
enabled = true
list = true


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
    
