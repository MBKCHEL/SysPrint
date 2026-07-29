# SysPrint

A fast, lightweight, and customizable system information fetch tool written in Rust. Inspired by `neofetch` and `fastfetch`.

## 💡 Usage Note

> **Note for Windows & Terminal Users:**  
> After displaying the system information, `SysPrint` waits for you to press **`Enter`** before exiting. 
> * **If you run it from terminal (CMD / PowerShell / Bash):** Just press `Enter` to return to your command line.
> * **If you run it by double-clicking the `.exe` on Windows:** Press `Enter` when you are done reading your stats so the window closes.

## Features
- 🚀 Blazing fast performance thanks to Rust
- 🎨 Beautiful ASCII art logos and colored CLI output
- 💻 Displays CPU, RAM, OS, Kernel, and Hostname info using the `sysinfo` crate

## Installation

### Linux

1. **Clone the repository:**
   ```bash
   git clone https://github.com/MBKCHEL/SysPrint.git
   cd SysPrint

2. Build the release binary:
   ```
   cargo build --release
   ```
3. Install globally (optional):
Move the binary to /usr/local/bin to run sysprint from anywhere in your terminal:
```
sudo cp target/release/sysprint /usr/local/bin/
```

4. Auto-run on Terminal Startup (optional):
   To display system info every time you open a terminal, add sysprint to your shell config file (~/.bashrc or ~/.zshrc):
```
echo "sysprint" >> ~/.bashrc
```

### Windows
### Download last release on GitHub, and run his!


