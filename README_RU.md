[English](README.md) | [Русский](README_RU.md)

# SysPrint

Быстрая, легковесная и гибко настраиваемая утилита для вывода информации о системе, написанная на Rust. Вдохновлена `neofetch` и `fastfetch`.

---
## Контрибьюторы

* [@dev-er1](https://github.com/dev-er1) — первый контрибьютор, респект 
* [@MBKCHEL](https://github.com/MBKCHEL) — создатель проекта (owner)
* [@BALBES](https://github.com/BALB3S) — QA и лучший друг)))
---
## 🖼️ Скриншоты и превью

Windows <img width="972" height="494" alt="изображение" src="https://github.com/user-attachments/assets/b266bc19-46ad-45bc-b08b-61df368fc4f8" />
Debian <img width="1005" height="570" alt="изображение" src="https://github.com/user-attachments/assets/95c5627f-4b99-4a92-9fbd-28b8b025d149" />
Mint <img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/f748baf5-4be8-4286-b759-280d0ad794e5" />
Arch (ПК) <img width="954" height="1037" alt="изображение" src="https://github.com/user-attachments/assets/33568879-8d47-4d50-a828-511879f0280d" />
Arch (Ноутбук) <img width="1142" height="801" alt="image" src="https://github.com/user-attachments/assets/b7e8165b-32a3-460b-8bb1-4bdfacf3f765" />
Artix <img width="1254" height="759" alt="image" src="https://github.com/user-attachments/assets/1ce05a92-367c-4d50-a314-662a2cc913df" />
NixOS <img width="1201" height="766" alt="изображение" src="https://github.com/user-attachments/assets/a37252bb-5167-48c0-a14a-fe7d790f050a" />

> 🐧 **Поддерживаемые логотипы:** AlmaLinux, Alpine, Alt Linux, Android, Apple (macOS), Arch, Artix, Asahi, Astra Linux, CachyOS, CentOS, ChromeOS, Debian, Deepin, Elementary OS, EndeavourOS, Fedora, FreeBSD, Garuda, Gentoo, Kali Linux, KDE neon, Kubuntu, Lubuntu, Manjaro, Linux Mint, MX Linux, NetBSD, NixOS, OpenBSD, openSUSE, Parrot OS, Pop!_OS, Proxmox, Raspberry Pi OS, RHEL, Rocky Linux, Slackware, Solus, SteamOS, Tails, TrueNAS, Tux, Ubuntu, Void Linux, Windows, Xubuntu, Zorin OS. Список постоянно пополняется!  
> *Если ваш дистрибутив пока не поддерживается явно, SysPrint автоматически использует стандартный логотип пингвина Tux.*

---
### Конфигурация

* **Linux / BSD:** `~/.config/sysprint/config.toml`
* **Windows:** `%APPDATA%\sysprint\config.toml` *(обычно `C:\Users\Имя\AppData\Roaming\sysprint\config.toml`)*

### Пример файла `config.toml`

```toml
# Конфигурация SysPrint

# Принудительная замена логотипа по имени (например: "arch", "debian", "ubuntu", "fedora", "windows", "tux", "apple", "centos" и т.д.)
# Оставьте пустым для автоопределения системы.
logo = ""

# Настройка цветов
# Доступные цвета: "auto", "cyan", "blue", "green", "red", "magenta", "yellow", "white", "black"
# "auto" использует фирменный цвет логотипа текущего дистрибутива.
accent-color = "auto"
header-color = "auto"

# Режимы работы
mini-logo-mode = false
fast-mode = false
compact-mode = false
show-progress-bars = false
config-stronger = false

# Главные переключатели секций
show-system-info = true
show-cpu-info = true
show-gpu-info = true
show-memory-info = true
show-other-info = true
show-disks-info = true

# Построчные переключатели: Система
show-os = true
show-kernel = true
show-os-version = true
show-init = true
show-host = true
show-user = true
show-uptime = true
show-load-avg = true
show-processes = true

# Построчные переключатели: Процессор (CPU)
show-cpu-name = true
show-cpu-freq = true
show-cpu-usage = false # Замер нагрузки процессора требует задержки ~200ms
show-cpu-temp = true
show-cpu-cores = true
show-cpu-arch = true

# Построчные переключатели: Видеокарта (GPU)
show-gpu-name = true
show-gpu-temp = true
show-gpu-vram = true

# Построчные переключатели: Память
show-ram = true
show-swap = true

# Построчные переключатели: Прочее
show-de = true
show-wm = true
show-terminal = true
show-shell = true
show-resolution = true
show-local-ip = true
show-battery = true
show-locale-time = true
show-fetch-info = true

# Построчные переключатели: Диски
show-disks = true
show-all-disks = false
```

## Аргументы командной строки (CLI)

| Флаг / Опция | Описание |
|---|---|
| `--logo <NAME\|PATH>` | Принудительно установить логотип (например: `debian`, `arch`, `ubuntu`, `fedora`, `windows`, `apple`, `tux`, `centos` и т.д.) или указать путь к текстовому файлу (`./art.txt`) |
| `-C, --color <COLOR>` | Переопределить акцентный цвет (`auto`, `cyan`, `blue`, `green`, `red`, `magenta`, `yellow`, `white`, `black`) |
| `-b, --bars` | Отображать графические шкалы прогресса для RAM, Swap, CPU, дисков и батареи |
| `--cpu-usage` | Измерить и отобразить процент нагрузки процессора (требует задержки выборки ~200ms) |
| `--all-disks` | Отображать все точки монтирования, включая виртуальные, loop и контейнерные ФС |
| `--time` | Отображать время выполнения в миллисекундах рядом с версией SysPrint |
| `--no-logo` | Не отображать ASCII-логотип (вывод только текста без отступа слева) |
| `--json` | Вывести все собранные данные о системе в формате JSON и выйти |
| `--mini` | Отображать компактный 5-строчный mini ASCII-логотип |
| `--fast-mode` | Пропустить ресурсоемкие вызовы (диски, сетевые интерфейсы, подпроцессы шелла) для мгновенного запуска |
| `--compact-mode` | Компактный режим вывода без разделителей и подробных деталей |
| `--config-path` | Вывести путь к файлу конфигурации и выйти |
| `--generate-config` | Сгенерировать новый дефолтный `config.toml` в каталоге пользователя |
| `--no-pause` | Windows: не ждать нажатия клавиши Enter перед выходом |
| `--hide-system` | Скрыть секцию System |
| `--hide-cpu` | Скрыть секцию CPU |
| `--cpu-usage` | Отображать процент загрузки CPU (требует задержки ~200ms для замера) |
| `--hide-gpu` | Скрыть секцию GPU |
| `--hide-memory` | Скрыть секцию памяти и Swap |
| `--hide-other` | Скрыть секцию прочей информации (DE, WM, Shell, IP, батарея) |
| `--hide-disks` | Скрыть секцию дисков |
| `--hide-fetch-info` | Скрыть нижнюю строчку "SysPrint vX.Y.Z" |

## Возможности
- 🚀 Высочайшая скорость работы благодаря Rust
- 🎨 Красивые цветные ASCII-логотипы и аккуратный терминальный вывод
- 💻 Сбор информации о CPU, RAM, Swap, GPU, ОС, процессах, IP-адресах, дисках и батарее с помощью крейта `sysinfo`

---

## Установка

### Linux

#### Вариант 1: Быстрая установка (готовый бинарник)
Скачайте актуальный бинарный файл со [страницы релизов](https://github.com/MBKCHEL/SysPrint/releases/latest) и установите его:
```bash
chmod +x ~/Downloads/sysprint-linux
sudo mv ~/Downloads/sysprint-linux /usr/local/bin/sysprint
```
*Или если у вас русскоязычная система:*
```bash
chmod +x ~/Загрузки/sysprint-linux
sudo mv ~/Загрузки/sysprint-linux /usr/local/bin/sysprint
```
Запуск:
```bash
sysprint
```

#### Вариант 2: Сборка из исходников
```bash
git clone https://github.com/MBKCHEL/SysPrint.git
cd SysPrint
cargo build --release
sudo cp target/release/sysprint /usr/local/bin/
```

#### Автозапуск при открытии терминала (по желанию)
```bash
# Для Bash
echo "sysprint" >> ~/.bashrc

# Для Zsh
echo "sysprint" >> ~/.zshrc

# Для Fish
echo "sysprint" >> ~/.config/fish/config.fish
```

### FreeBSD / OpenBSD / NetBSD
Скачайте бинарник `sysprint-freebsd` со страницы релизов:
```bash
chmod +x ~/Downloads/sysprint-freebsd
sudo mv ~/Downloads/sysprint-freebsd /usr/local/bin/sysprint
sysprint
```

### Windows
Скачайте исполняемый файл `.exe` со страницы релизов и запустите.

---

## Удаление

### Linux и BSD (FreeBSD, OpenBSD, NetBSD)
1. Удалите исполняемый файл:
   ```bash
   sudo rm /usr/local/bin/sysprint
   ```
2. Удалите из автозапуска терминала (если добавляли):
   ```bash
   # Bash
   sed -i '/sysprint/d' ~/.bashrc

   # Zsh
   sed -i '/sysprint/d' ~/.zshrc

   # Fish
   fish -c "sed -i '/sysprint/d' ~/.config/fish/config.fish"
   ```

### Windows
1. Удалите скачанный файл `sysprint.exe`.
2. Если вы добавляли программу в автозагрузку, нажмите `Win + R`, введите `shell:startup` и удалите ярлык `sysprint`.
