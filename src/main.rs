use std::env;
use std::fs;
use std::process::Command;
use std::io::stdin;
use sysinfo::{System, Components, Disks};
use colored::*;


fn get_gpu_info() -> Vec<String> {
    let mut gpu_lines = Vec::new();

    // Temp And Vram
    let nvidia_output = Command::new("nvidia-smi")
        .args([
            "--query-gpu=gpu_name,memory.total,memory.used,temperature.gpu",
            "--format=csv,noheader,nounits"
        ])
        .output();

    if let Ok(out) = nvidia_output {
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !stdout.is_empty() {
            let parts: Vec<&str> = stdout.split(',').map(|s| s.trim()).collect();
            if parts.len() >= 4 {
                let name = parts[0];
                let mem_total: f64 = parts[1].parse().unwrap_or(0.0) / 1024.0;
                let mem_used: f64 = parts[2].parse().unwrap_or(0.0) / 1024.0;
                let temp = parts[3];

                gpu_lines.push(format!("{}: {}", "GPU".bold(), name));
                gpu_lines.push(format!("{}: {:.2} GB / {:.2} GB", "VRAM".bold(), mem_used, mem_total));
                gpu_lines.push(format!("{}: {}°C", "GPU Temp".bold(), temp));

                return gpu_lines;
            }
        }
    }

    // If not nvidia we give name.
    let mut name = String::new();

    #[cfg(windows)]
    {
        let output = Command::new("powershell")
            .args(["-Command", "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name"])
            .output();

        if let Ok(out) = output {
            let gpu = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !gpu.is_empty() {
                name = gpu.lines().next().unwrap_or("Unknown GPU").to_string();
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("sh")
            .arg("-c")
            .arg("lspci | grep -Ei 'vga|3d|display'")
            .output();

        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Some(line) = text.lines().next() {
                if let Some(pos) = line.find(':') {
                    let raw_name = line[pos + 1..].trim();
                    name = raw_name.split(':').last().unwrap_or(raw_name).trim().to_string();
                }
            }
        }
    }

    if name.is_empty() {
        name = "Unknown GPU".to_string();
    }

    gpu_lines.push(format!("{}: {}", "GPU".bold(), name));
    gpu_lines
}


fn main() {
    // For fucking Windows CMD
    #[cfg(windows)]
    let _ = control::set_virtual_terminal(true);

    let mut sys = System::new_all();
    sys.refresh_all();
    sys.refresh_cpu_all();

    // 1. Collect all system information into a vector of strings
    let mut info: Vec<String> = Vec::new();

    // --- SYSTEM INFO ---
    info.push(format!("{}", "--- System INFO ---".bold().cyan()));
    info.push(format!("{}: {}", "OS".bold(), System::name().unwrap_or_default()));
    info.push(format!("{}: {}", "OS Version".bold(), System::os_version().unwrap_or_default()));
    info.push(format!("{}: {}", "Host".bold(), System::host_name().unwrap_or_default()));


    // --- CPU INFO ---
    info.push(format!("{}", "--- CPU INFO ---".bold().cyan()));

    // GHz and name CPU
    let cpus = sys.cpus();
    if let Some(cpu) = cpus.first() {
        info.push(format!("{}: {}", "CPU".bold(), cpu.brand().trim()));
        let freq_ghz = cpu.frequency() as f64 / 1000.0;
        info.push(format!("{}: {:.2} GHz", "Freq".bold(), freq_ghz));
    } else {
        info.push("CPU: Unknown".to_string());
    }

    // CPU Usage
    let global_cpu = sys.global_cpu_usage();
    (&mut info).push(format!("{}: {:.1}%", "CPU Usage".bold(), global_cpu));

    // CPU Temperature
    let components = Components::new_with_refreshed_list();
    let cpu_temp = components.iter().find_map(|comp| {
        let label = comp.label().to_lowercase();
        if label.contains("cpu") || label.contains("core") || label.contains("package") || label.contains("k10temp") {
            comp.temperature()
        } else {
            None
        }
    });

    if let Some(temp) = cpu_temp {
        info.push(format!("{}: {:.1}°C", "CPU Temp".bold(), temp));
    }

    // Processor Cores & Threads
    info.push(format!("{}: {}", "Cores".bold(), System::physical_core_count().unwrap_or(0)));
    info.push(format!("{}: {}", "Threads".bold(), cpus.len()));


    // --- MEMORY INFO ---
    info.push(format!("{}", "--- Memory INFO ---".bold().cyan()));
    let total_ram = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_ram = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    info.push(format!("{}: {:.2} GB / {:.2} GB", "RAM".bold(), used_ram, total_ram));

    // --- DISKS INFO ---
    info.push(format!("{}", "--- Disks INFO ---".bold().cyan()));
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total_gb = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let available_gb = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = total_gb - available_gb;

        let mount_point = disk.mount_point().to_string_lossy();
        info.push(format!(
            "{}: {:.2} GB / {:.2} GB ({})",
            mount_point.bold(),
            used_gb,
            total_gb,
            disk.file_system().to_string_lossy()
        ));
    }

    // --- Other Info ---
    info.push(format!("{}", "--- Other Info ---".bold().cyan()));

    // 1. Desktop Environment / Window Manager
    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "Unknown".to_string());
    info.push(format!("{}: {}", "DE/WM".bold(), desktop));

    // 2. Battery status (checks /sys/class/power_supply/BAT0 or BAT1)
    let battery = match (
        fs::read_to_string("/sys/class/power_supply/BAT0/capacity"),
        fs::read_to_string("/sys/class/power_supply/BAT0/status"),
    ) {
        (Ok(cap), Ok(stat)) => format!("{}% [{}]", cap.trim(), stat.trim()),
        _ => match (
            fs::read_to_string("/sys/class/power_supply/BAT1/capacity"),
            fs::read_to_string("/sys/class/power_supply/BAT1/status"),
        ) {
            (Ok(cap), Ok(stat)) => format!("{}% [{}]", cap.trim(), stat.trim()),
            _ => "N/A (Desktop)".to_string(),
        },
    };
    info.push(format!("{}: {}", "Battery".bold(), battery));


    // TODO: Render logic with ASCII Logo goes here
    // GPU info
    info.push(format!("{}", "--- GPU INFO ---".bold().cyan()));
    info.extend(get_gpu_info());


    // --- Render logic with ASCII Logo ---
    let (logo, logo_padding) = get_logo();

    let max_lines = std::cmp::max(logo.len(), info.len());

    for i in 0..max_lines {

        if i < logo.len() {
            let logo_line = &logo[i];


            print!("{}", logo_line);


            let raw_len = strip_ansi_escapes::strip_str(&logo_line.to_string()).chars().count();


            let pad = if logo_padding > raw_len {
                logo_padding - raw_len + 3
            } else {
                3
            };
            print!("{}", " ".repeat(pad));
        } else {

            print!("{}", " ".repeat(logo_padding + 3));
        }


        let info_line = info.get(i).unwrap_or(&String::new()).clone();
        println!("{}", info_line);
    }

    stdin().read_line(&mut String::new()).expect("Did not enter a correct string");
    fn get_logo() -> (Vec<ColoredString>, usize) {
        // Give Name OS
        let os_name = System::name().unwrap_or_default().to_lowercase();
        if os_name.contains("arch") {
            //Arch (I Use Arch BTW)
            let logo = vec![
                "                        .+.                       ".blue().bold(),
                "                        +++                       ".blue().bold(),
                "                       .+++.                      ".blue().bold(),
                "                      .+++++.                     ".blue().bold(),
                "                     .+++++++.                    ".blue().bold(),
                "                    .+++++++++.                   ".blue().bold(),
                "                     ++++++++++                   ".blue().bold(),
                "                  .+...+++++++++.                 ".blue().bold(),
                "                  ++++++.*+++++++                 ".blue().bold(),
                "                .+++++++++++++++++.               ".blue().bold(),
                "                +++++++++++++++++++               ".blue().bold(),
                "              .+++++++++++++++++++++.             ".blue().bold(),
                "             .+++++++++++++++++++++++.            ".blue().bold(),
                "            .++++++++++.    .+++++++++            ".blue().bold(),
                "           .+++++++++.        +++++++++           ".blue().bold(),
                "          .+++++++++*         .+++++++++.         ".blue().bold(),
                "        .++++++++++++         .+++++++. *+        ".blue().bold(),
                "        +++++++++++++         .++++++++++..       ".blue().bold(),
                "      .+++++++++++++*.        +++++++++++++*.     ".blue().bold(),
                "     .+++++++..                      .+++++++.    ".blue().bold(),
                "    ++++.                                  ++++   ".blue().bold(),
                "  .+                                          .*. ".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
            ];
            (logo, 16) // возвращаем лого и ширину отступа (16 пробелов)

        } else if os_name.contains("ubuntu") {
            // Bubuntu
            let logo = vec![
                "                                     #+                   ".yellow().bold(),
                "                                  #######=                ".yellow().bold(),
                "                          :-:    =########                ".yellow().bold(),
                "                    ++++++++++++  ########                ".yellow().bold(),
                "                --   ++++++++++++   *##                   ".yellow().bold(),
                "              -----   ++++++++++++++= .++                 ".red().bold(),
                "            --------            +++++++++++               ".red().bold(),
                "           ---------               +++++++++              ".red().bold(),
                "          --------.                 -+++++++=             ".red().bold(),
                "    +++++   -----                    -+++++++             ".red().bold(),
                "   ++++++++  ----                                         ".red().bold(),
                "  =++++++++  ----                                         ".red().bold(),
                "   +++++++  -----                     #######             ".red().bold(),
                "           -------                   ########             ".red().bold(),
                "           --------                 ########              ".red().bold(),
                "            --------.            ##########               ".red().bold(),
                "             ------    ###################                ".yellow().bold(),
                "               ---    ############                        ".yellow().bold(),
                "                    *###########  .------                 ".yellow().bold(),
                "                      ########## .--------                ".yellow().bold(),
                "                                  --------                ".yellow().bold(),
                "                                   -----.                 ".yellow().bold(),
                "                                                          ".blue().bold(),
                "                                                          ".blue().bold(),
                "                                                          ".blue().bold(),
                "                                                          ".blue().bold(),
            ];
            (logo, 16)
        } else if os_name.contains("windows") {
            //Windows
            let logo = vec![
                "                                                  .".blue().bold(),
                "                                    .*************".blue().bold(),
                "                       ***************************".blue().bold(),
                "        =************  ***************************".blue().bold(),
                "*********************  ***************************".blue().bold(),
                "*********************  ***************************".blue().bold(),
                "*********************  ***************************".blue().bold(),
                "*********************  ***************************".blue().bold(),
                "*********************  ***************************".blue().bold(),
                "*********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                ":::::....                                         ".normal(),
                "====------::::::....                              ".normal(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "+********************  ***************************".blue().bold(),
                "        :************  ***************************".blue().bold(),
                "                       :**************************".blue().bold(),
                "                                     .************".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
            ];
            (logo, 19)
        } else if os_name.contains("darwin") || os_name.contains("mac") || os_name.contains("macos") {
            // macOS Apple Logo
            let logo = vec![
                "                                                  ".white().bold(),
                "                              @@@@@               ".white().bold(),
                "                            @@@@@@@               ".white().bold(),
                "                           @@@@@@@                ".white().bold(),
                "                          @@@@@@@                 ".white().bold(),
                "                         @@@@@%                   ".white().bold(),
                "                                                  ".white().bold(),
                "           :@@@@@@@@@@@@   *@@@@@@@@@@@@@=        ".white().bold(),
                "         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@      ".white().bold(),
                "       :@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@       ".white().bold(),
                "      *@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         ".white().bold(),
                "      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          ".white().bold(),
                "     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           ".white().bold(),
                "     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           ".white().bold(),
                "     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@           ".white().bold(),
                "     @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@          ".white().bold(),
                "      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         ".white().bold(),
                "      @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@        ".white().bold(),
                "       @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#     ".white().bold(),
                "       @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@     ".white().bold(),
                "        @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@     ".white().bold(),
                "         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@.      ".white().bold(),
                "           @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@        ".white().bold(),
                "            @@@@@@@@@@@@@@@@@@@@@@@@@@@@@         ".white().bold(),
                "              @@@@@@@@@@@* *@@@@@@@@@@@           ".white().bold(),
            ];
            (logo, 39)
        } else if os_name.contains("mint") {
            //mint
            let logo = vec![
                "                       .===.                      ".green().bold(),
                "                ===================               ".green().bold(),
                "             =========================            ".green().bold(),
                "           =============================          ".green().bold(),
                "         =================================        ".green().bold(),
                "       :====....===========================:      ".green().bold(),
                "      ======....=====................========     ".green().bold(),
                "      ======....====...................======     ".green().bold(),
                "     =======....====....===....====....=======    ".green().bold(),
                "     =======....====....===....====....=======    ".green().bold(),
                "    ========....====....===....====....========   ".green().bold(),
                "    ========....====....===....====....========   ".green().bold(),
                "     =======....====....===....====....=======    ".green().bold(),
                "     =======....===================....=======    ".green().bold(),
                "      ======....===================....======     ".green().bold(),
                "      =======.........................=======     ".green().bold(),
                "       :=======.....................=======:      ".green().bold(),
                "         =================================        ".green().bold(),
                "           =============================          ".green().bold(),
                "             =========================            ".green().bold(),
                "                ===================               ".green().bold(),
                "                       .===.                      ".green().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
            ];
            (logo, 19)
        } else if os_name.contains("gentoo") {
            //gentoo
            let logo = vec![
                "                         -=.                                 ".white().bold(),
                "                ====.     ...=====                          ".white().bold(),
                "            ===           .......:====                      ".white().bold(),
                "          ==               .......:::====                   ".white().bold(),
                "       .==                 .......::::::====                ".white().bold(),
                "      ==                    .......::::::-----=             ".white().bold(),
                "    ==                      .:--...::::::-------=           ".white().bold(),
                "   ==                        .:-===:::::::---------.        ".white().bold(),
                "  ==                  .   =========*#:::::--------:::=      ".white().bold(),
                "  ==-                  ---========*###::::----------::::    ".white().bold(),
                "  =---                  ===    ===###::::-----------=-...   ".white().bold(),
                "  ===----                  ########::::::-----------===...= ".white().bold(),
                "   =====--:::               ......:::::::-----------====   =".white().bold(),
                "     ======::::::          .......:::::::----------=====   =".white().bold(),
                "        =======......     .......:::::::-----------====    =".white().bold(),
                "            =====...      .......:::::::-----------===    ==".white().bold(),
                "               .         .......:::::::-----------==    ====".white().bold(),
                "            =           .......::::::::-----------    .==== ".white().bold(),
                "                      ........::::::::---------      ====   ".white().bold(),
                "                     ........::::::::--------     +++++     ".white().bold(),
                "                   .........::::::::------      +++++       ".white().bold(),
                "                 ..........::::::::-----     .+++++         ".white().bold(),
                "              ...........:::::::::---      +++++:           ".white().bold(),
                " =        .............:::::::::--      ++++++              ".white().bold(),
                " +  .................:::::::::.      ++++++                 ".white().bold(),
                "++  ..............:::::::::      ++++****                   ".white().bold(),
                " +   ..........::::::::       ********                      ".white().bold(),
                " ++     ..::::::         *********                          ".white().bold(),
                "  +++               ***********                             ".white().bold(),
                "   ************************                                 ".white().bold(),
                "      ****************                                      ".white().bold()
            ];
            (logo, 19)
        } else if os_name.contains("debian") {
            //debian
            let logo = vec![
                "                                                  ".red().bold(),
                "                  %%%%%%%%%%%%%%                  ".red().bold(),
                "              %%%%%%%%%%%%%%%%%%%%%%%%            ".red().bold(),
                "           .%%%%%%%%#           %%%%%%%%          ".red().bold(),
                "          %%%%%%                   %%%%%%%        ".red().bold(),
                "        #%%%%%                       %%%%%%       ".red().bold(),
                "       *%%%                           %%%%%%      ".red().bold(),
                "      %%%*               .%%%.         %%%%       ".red().bold(),
                "     %%%%             %%        *       %%%       ".red().bold(),
                "     %%%             %                  %%%       ".red().bold(),
                "     %%             %                   %%%       ".red().bold(),
                "    .%%            :%                   %%        ".red().bold(),
                "    .%%             %                   %%        ".red().bold(),
                "    :%%             %%                %%%         ".red().bold(),
                "     %%.           : #%              %%           ".red().bold(),
                "     %%%            .. #%%        %%%             ".red().bold(),
                "     *%%                %- %%%%%%                 ".red().bold(),
                "      %%%%#                                       ".red().bold(),
                "       %%%%                                       ".red().bold(),
                "        %%%*                                      ".red().bold(),
                "         %%%%                                     ".red().bold(),
                "           %%%                                    ".red().bold(),
                "             %%%                                  ".red().bold(),
                "               %%%                                ".red().bold(),
                "                  %%%%                            ".red().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
                "                                                  ".blue().bold(),
            ];
            (logo, 19)

        } else if os_name.contains("nixos") {
            //nixOS
            let logo = vec![
                "                      ....        ........        ....                     ".cyan().bold(),
                "                     ++++++        --------      ------                    ".cyan().bold(),
                "                   .++++++++        --------    --------.                  ".cyan().bold(),
                "                     ++++++++        --------. --------                    ".cyan().bold(),
                "                      ++++++++        ----------------                     ".cyan().bold(),
                "                       ++++++++        .-------------                      ".cyan().bold(),
                "               +++++++++++++++++++++++++*-----------        +              ".cyan().bold(),
                "              ++++++++++++++++++++++++++++--------         +++             ".cyan().bold(),
                "             ++++++++++++++++++++++++++++++--------      ++++++            ".cyan().bold(),
                "            ============++++++++============--------.   ++++++++           ".cyan().bold(),
                "                       --------              --------  ++++++++            ".cyan().bold(),
                "                     .--------                --------++++++++             ".cyan().bold(),
                "                    --------.                  .-----++++++++              ".cyan().bold(),
                "       --------------------.                    .---+++++++++++++++++      ".cyan().bold(),
                "     .--------------------                        ++++++++++++++++++++     ".cyan().bold(),
                "      -------------------++                      +++++++++++++++++++++     ".cyan().bold(),
                "       -----------------++++                    +++++++++++++++++++++      ".cyan().bold(),
                "               --------++++++                  ++++++++                    ".cyan().bold(),
                "             ---------++++++++               .++++++++                     ".cyan().bold(),
                "            --------   ++++++++.            .++++++++                      ".cyan().bold(),
                "            -------.    ++++++++--------------------------------           ".cyan().bold(),
                "             -----       +++++++++-----------------------------            ".cyan().bold(),
                "              ---        +++++++++*---------------------------             ".cyan().bold(),
                "               -        ++++++++++++------------------------               ".cyan().bold(),
                "                      .++++++++++++++        --------.                     ".cyan().bold(),
                "                      ++++++++++++++++.       --------                     ".cyan().bold(),
                "                    +++++++++  ++++++++        ---------                   ".cyan().bold(),
                "                    +++++++     ++++++++         -------                   ".cyan().bold(),
                "                     +++++       =++++++++        -----                    ".cyan().bold(),

            ];
            (logo, 19)

        } else if os_name.contains("manjaro") {
            //manjaro
            let logo = vec![
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++++++++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.                       .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  -------------------  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ...................  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "                           +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
                "      ++++++++++++++++++.  +++++++++++++++++++  .++++++++++++++++++".green().bold(),
            ];
            (logo, 19)

        } else if os_name.contains("void") {
            //void
            let logo = vec![
                "                                           ...............                                          ".cyan().bold(),
                "                                     ..   .............. ... ...                                    ".cyan().bold(),
                "                                .......:---------------------.......                                ".cyan().bold(),
                "                               ....------------------------------:. ...                             ".cyan().bold(),
                "                             ...-------------------------------------... .                          ".cyan().bold(),
                "                              .  .-------------------------------------.. ..                        ".cyan().bold(),
                "                               .....-------------::----------------------... .                      ".cyan().bold(),
                "                       .....     .....---....          ....:---------------.. .                     ".cyan().bold(),
                "                     ...*:....      ........             .. ...-------------:...                    ".cyan().bold(),
                "                   ....****:.. .                            .....-------------...                   ".cyan().bold(),
                "                  ....*******:. ..                             ....------------. .                  ".cyan().bold(),
                "                  ...**********:.           ............         . .:-----------. .                 ".cyan().bold(),
                "                 . .***********...       .......-----......       ....----------...                 ".cyan().bold(),
                "                ....**********... .    .  ..-------------..         ...----------...   ...          ".cyan().bold(),
                "         . .@@@@@@@@@********:.....@@@..@@@@@@@@@@@@@@@---:.@@@@@@@..@@@@@@@@@@@@@@@@@@. ..         ".cyan().bold(),
                "          .. @@@@@@@@@*******. .@@@..@@@@@@@-----%@@@@@@---@@@@@@@=.@@@@@@@#-----@@@@@@@*..         ".cyan().bold(),
                "           .. @@@@@@@@@+*****%@@   @@@@@@@=------@@@@@@@=-@@@@@@@#.@@@@@@@@-----*@@@@@@@@ .         ".cyan().bold(),
                "            ...@@@@@@@@@**%@@..   @@@@@@@*------@@@@@@@@-@@@@@@@@.%@@@@@@@-----=@@@@@@@@...         ".cyan().bold(),
                "             .. @@@@@@@@@@***.. . @@@@@@@------@@@@@@@@-%@@@@@@@.=@@@@@@@-----=@@@@@@@@ .           ".cyan().bold(),
                "              .  @@@@@@******..   @@@@@@@@@@@@@@@@@@#--+@@@@@@@..@@@@@@@@@@@@@@@@@@@@.              ".cyan().bold(),
                "               ...:#*********+..    .=====+****+-------***==+=. ==++===*********-......             ".cyan().bold(),
                "                .. .**********. .......... .:-----------... ........ .-----------...                ".cyan().bold(),
                "                 . .+**********.. .       ...............         ...-----------...                 ".cyan().bold(),
                "                  . .***********:....         ........            ....---------:. .                 ".cyan().bold(),
                "                   ...************.....                            .....------:...                  ".cyan().bold(),
                "                    . .*************.....                   ...      .. ..---.. .                   ".cyan().bold(),
                "                     ...=*************+.. . ..         ... . .. ..      .. ... .                    ".cyan().bold(),
                "                      ....****************=...............*****:....      .. .                      ".cyan().bold(),
                "                        .. .*************************************.....                              ".cyan().bold(),
                "                          ....*************************************:...                             ".cyan().bold(),
                "                            ....:***********************************..                              ".cyan().bold(),
                "                              .... .****************************-.....                              ".cyan().bold(),
                "                                  .......+*****************-.... ..                                 ".cyan().bold(),
                "                                      . ................ .....                                      ".cyan().bold(),
                "                                                 ...                                                ".cyan().bold(),

            ];
            (logo, 19)

        } else if os_name.contains("kali") {
            //Kali Linus
            let logo = vec![
                "              :%@@@@@@@@@@*.                                                          ".black().bold(),
                "                          .#@@@@@@@.                                                  ".black().bold(),
                "                                  +@@@@@                                              ".black().bold(),
                "                      ..-=**#%%@@%#*+-@@                                              ".black().bold(),
                "   :%@@@@@@*-.                         @                                              ".black().bold(),
                "                              .%@@@@@@@@                                              ".black().bold(),
                "                      .@@@@@.           @                                             ".black().bold(),
                "                .@@@.                    @@@                                          ".black().bold(),
                "           .@@.                           @@@@@@@@@@@@+                               ".black().bold(),
                "       #@                              @@@@@.       :@@@@@@.@                         ".black().bold(),
                "   +=                                @@@@                 @@@@@*@                     ".black().bold(),
                "-                                   @@@                       .@@@@@%                 ".black().bold(),
                "                                   #@@                           @@@:@                ".black().bold(),
                "                                   @@@                            .@@@%               ".black().bold(),
                "                                   @@*                                @@@@+           ".black().bold(),
                "                                   @@@                                                ".black().bold(),
                "                                   @@@:                                               ".black().bold(),
                "                                    @@@#                                              ".black().bold(),
                "                                     @@@@+                                            ".black().bold(),
                "                                       %@@@@@@%=-..                                   ".black().bold(),
                "                                             =@@@@@@@@@@@@@@@@@@@@:                   ".black().bold(),
                "                                                           .@@@@%@@@@@                ".black().bold(),
                "                                                                @@+   :@@:            ".black().bold(),
                "                                                                   @@     @@          ".black().bold(),
                "                                                                     @=     :@        ".black().bold(),
                "                                                                       @      %.      ".black().bold(),
                "                                                                        @:      -     ".black().bold(),
                "                                                                         ##           ".black().bold(),
                "                                                                          #+          ".black().bold(),
                "                                                                           @          ".black().bold(),
                "                                                                            @         ".black().bold()
            ];
            (logo, 16)
        } else if os_name.contains("fedora") {
            //Fedora
            let logo = vec![
                "                      .... ...                          ".blue().bold(),
                "                 .********#######..                     ".blue().bold(),
                "            ..**********############*.                  ".blue().bold(),
                "          ..**********#################.                ".blue().bold(),
                "         .**********########....+########.              ".blue().bold(),
                "         *********#######:.       .#######.             ".blue().bold(),
                "        ********######### ..####.. .#######.            ".blue().bold(),
                "       *******##########:. .####%. .#######*            ".blue().bold(),
                "      .*****############:. .####.  %########.           ".blue().bold(),
                "       ***#########=::##:  .::=#############.           ".blue().bold(),
                "      .*#######.   . .%#:.     .############.           ".blue().bold(),
                "      .*######. ..######:  .################.           ".blue().bold(),
                "      .*#####.  ########:. .################            ".blue().bold(),
                "      .*#####.  ########. .:###############             ".blue().bold(),
                "      .######:  .######-. .###############.             ".blue().bold(),
                "      .########.   ..    .###############.              ".blue().bold(),
                "      .##########=.....%###############.                ".blue().bold(),
                "      .##############################.                  ".blue().bold(),
                "       :#########################..                     ".blue().bold(),
                "         . ..................                           ".blue().bold(),
                "                                                        ".blue().bold(),
                "                                                        ".blue().bold(),
                "                                                        ".blue().bold(),
                "                                                        ".blue().bold(),
                "                                                        ".blue().bold(),
                "                                                        ".blue().bold(),
            ];
            (logo, 19)
        } else {
            // Default logo GNU/Linux
            let logo = vec![
                "                         :@+.                     ".white().bold(),
                "                    @@@@@@@@@@@@@                 ".white().bold(),
                "                  .@@@@@@@@@@@@@@@                ".white().bold(),
                "                  @@@@@@@@@@@@@@@@@               ".white().bold(),
                "                 @@@@@@@@@@@@@@@@@@               ".white().bold(),
                "                 @@   @@@@     @@@@@              ".white().bold(),
                "                 @@ @@ @@  @@  @@@@@              ".white().bold(),
                "                 @@ @@-==--@@  @@@@@              ".white().bold(),
                "                 .@============@@@@@              ".white().bold(),
                "                 @@=@======@==@@@@@@              ".white().bold(),
                "                 @@@@-=====@@@@@@@@@@             ".white().bold(),
                "                @@@@ @@@@@@    @@@@@@@            ".white().bold(),
                "              @@@@@*            @@@@@@@@          ".white().bold(),
                "             @@@@@@              @@@@@@@@.        ".white().bold(),
                "           @@@@@@@               :@@@@@@@@@       ".white().bold(),
                "          @@@@@@@                 @@@@@@@@@@      ".white().bold(),
                "         @@@@@@@                   @@@@@@@@@@     ".white().bold(),
                "        @@@@@@@                     @@@@@@@@@@    ".white().bold(),
                "        @@@@@@@                      @@@@@@@@@    ".white().bold(),
                "       @@@@@@@:                     @@@@@@@@@@    ".white().bold(),
                "       @@@@@@@                     @@@@@@@@@@@    ".white().bold(),
                "      @@--:@@@@@                 @@@@@@@@@@@@@    ".white().bold(),
                "    @@@-----@@@@@@@              @@--@@@@@----@@  ".white().bold(),
                " @@----------@@@@@@@            +@@------------@  ".white().bold(),
                " @------------:@@@@@@           @@@------------+@ ".white().bold(),
                " @:-------------@@@@            @@@--------------@".white().bold(),
                " @---------------@@@@@       @@@@@--------------@@".white().bold(),
                "%@-----------------@@@@@@@@@@@@@@@----------@@@   ".white().bold(),
                " .@@@@@@@----------@@@@@@@@@@@@@@---------@@      ".white().bold(),
                "          @@@@@@@@@              @@----@@@        ".white().bold(),
            ];
            (logo, 16)
        }
    }
}