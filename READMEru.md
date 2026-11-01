# SysPrint

Быстрый, легковесный и настраиваемый инструмент для вывода системной информации, написанный на Rust. Создан под вдохновением от `neofetch` и `fastfetch`.

## 🎨 Галерея логотипов и кастомизация

Посмотрите файл [LOGOS.md](https://www.google.com/search?q=./LOGOS.md), чтобы увидеть все поддерживаемые логотипы операционных систем, мини-режимы и примеры пользовательского ASCII-арта.

> 🐧 **Поддерживаемые логотипы:** Alpine, Android, MacOS, Arch, Artix, Astra Linux, EndeavourOS, CachyOS, Debian, Fedora, FreeBSD, NetBSD, OpenBSD, OpenSUSE, Gentoo, Kali Linux, Manjaro, Linux Mint, NixOS, Pop!OS, Ubuntu, Void Linux, Zorin OS, Lubuntu, Xubuntu, Kubuntu и Windows. Скоро появится еще больше!
> _Если ваш дистрибутив пока не поддерживается явно, SysPrint переключится на стандартный логотип пингвина Такса (GNU/Linux)._

### Конфигурация

- **Linux / BSD:** `~/.config/sysprint/config.toml`
    
- **Windows:** `%APPDATA%\sysprint\config.toml` _(обычно `C:\Users\Имя\AppData\Roaming\sysprint\config.toml`)_
    

### Пример `~/.config/sysprint/config.toml`

```TOML
# Конфигурация SysPrint
# Когда флаг командной строки противоречит конфигу, параметр `config-stronger = true` делает приоритетным конфиг.
# Fast-mode отключает информацию о VRAM графического процессора и температуре.

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

## Если флаг командной строки противоречит конфигу, параметр `config-stronger = true` делает приоритетным конфиг
```bash
config-stronger = false
```

## Возможности

- 🚀 Потрясающая производительность благодаря Rust
    
- 🎨 Красивые логотипы из ASCII-арта и цветной вывод в командной строке
    
- 💻 Отображение информации о процессоре (CPU), оперативной памяти (RAM), ОС и ядре с помощью крейта `sysinfo`
    
## Установка

### Linux

#### Вариант 1: Быстрая установка (предкомпилированный бинарный файл)

Загрузите последнюю версию бинарного файла со [страницы релизов](https://github.com/MBKCHEL/SysPrint/releases/latest) и установите в папку загрузок:

```Bash
chmod +x ~/Downloads/sysprint-linux
sudo mv ~/Downloads/sysprint-linux /usr/local/bin/sysprint
```

Для запуска:

```Bash
sysprint
```

_Или если у вас русская локаль (папка загрузок называется «Загрузки»):_

```Bash
chmod +x ~/Загрузки/sysprint-linux
sudo mv ~/Загрузки/sysprint-linux /usr/local/bin/sysprint
```

#### Вариант 2: Сборка из исходного кода

```Bash
git clone https://github.com/MBKCHEL/SysPrint.git
cd SysPrint
cargo build --release
sudo cp target/release/sysprint /usr/local/bin/
```

##### Nix / NixOS

Для Nix или NixOS вы можете установить программу следующим образом:

1. Быстрый запуск

```Bash
nix run github:MBKCHEL/SysPrint
```

2. Через flake

```Nix
inputs = {
  nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  sysprint = {
    url = github:MBKCHEL/SysPrint;
  };
};
```

#### Автозапуск при старте терминала (опционально)

```Bash
# Для Bash
echo "sysprint" >> ~/.bashrc

# Для Zsh
echo "sysprint" >> ~/.zshrc

# Для Fish
echo "sysprint" >> ~/.config/fish/config.fish
```

### FreeBSD / OpenBSD / NetBSD

Вариант 1: Быстрая установка (предкомпилированный бинарный файл)

Скачайте бинарный файл `sysprint-freebsd` со страницы релизов:

```Bash
chmod +x ~/Downloads/sysprint-freebsd
sudo mv ~/Downloads/sysprint-freebsd /usr/local/bin/sysprint
```

Для запуска:

```Bash
sysprint
```

### Windows

Скачайте файл `.exe` со страницы релизов и запустите его.

## Удаление

### Linux и BSD (FreeBSD, OpenBSD, NetBSD)

1. Удалите бинарный файл:
    ```Bash
    sudo rm /usr/local/bin/sysprint
    ```
    
2. Удалите автозапуск из терминала (если он был добавлен):
3. 
    ```Bash
    # Bash
    sed -i '/sysprint/d' ~/.bashrc
    
    # Zsh
    sed -i '/sysprint/d' ~/.zshrc
    
    # Fish
    fish -c "sed -i '/sysprint/d' ~/.config/fish/config.fish"
    ```
    

### Windows

1. Удалите скачанный файл `sysprint.exe`.
    
2. Если программа была добавлена в автозагрузку, нажмите **Win + R**, введите `shell:startup` и удалите ярлык `sysprint`.