# SysPrint

A fast, lightweight, and customizable system information fetch tool written in Rust. Inspired by `neofetch` and `fastfetch`.

---

## 🖼️ Preview & Screenshots

Windows <img width="972" height="494" alt="изображение" src="https://github.com/user-attachments/assets/b266bc19-46ad-45bc-b08b-61df368fc4f8" />
Debian <img width="1005" height="570" alt="изображение" src="https://github.com/user-attachments/assets/95c5627f-4b99-4a92-9fbd-28b8b025d149" />
mint <img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/f748baf5-4be8-4286-b759-280d0ad794e5" />

> 🐧 **Supported Logos:** Arch, Debian, Linux Mint, Ubuntu, macOS, Gentoo, Fedora, Kali, and Windows. More coming soon!  
> *If your distro isn't explicitly supported yet, SysPrint will fall back to the standard GNU/Linux penguin Tux logo.*

---

## 💡 Usage Note

> **Note for Windows & Terminal Users:**  
> After displaying the system information, `SysPrint` waits for you to press **`Enter`** before exiting. 
> * **If you run it from terminal (CMD / PowerShell / Bash):** Just press `Enter` to return to your command line.
> * **If you run it by double-clicking the `.exe` on Windows:** Press `Enter` when you are done reading your stats so the window closes.

---

## Features
- 🚀 Blazing fast performance thanks to Rust
- 🎨 Beautiful ASCII art logos and colored CLI output
- 💻 Displays CPU, RAM, OS, Kernel, info using the `sysinfo` crate

---

## Installation

### Linux

#### Option 1: Fast Install (Precompiled Binary)
Download the latest binary from the (https://github.com/MBKCHEL/SysPrint/releases/tag/1.0.0) and install to download folder:
```bash
chmod +x ~/Downloads/sysprint-linux
sudo mv ~/Downloads/sysprint-linux /usr/local/bin/sysprint
```
*(Replace `Downloads` with `Загрузки` if needed).*

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

### Windows
Download the `.exe` from the Releases page and run it.
