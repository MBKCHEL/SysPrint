# Example home-manager configuration for SysPrint.
# Every option below is shown with its default value, except where noted.
{
  imports = [ inputs.sysprint.homeManagerModules.default ];

  programs.sysprint = {
    enable = true;

    settings = {
      # Logo
      logo = "default"; # or a distro name, e.g. "nixos", "arch"
      logo-color = "default"; # or red, green, blue, ...
      custom-logo-path = "none"; # e.g. "~/my_logo.txt"
      mini-logo-mode = false;

      # Behaviour
      config-stronger = false;
      fast-mode = false; # disables GPU VRAM and temperature info
      compact-mode = false;
      hide-fetch-info = false;
      show-sysprint-start-time = true;

      # Blocks: turn off a whole block or single fields
      system.enabled = true;
      cpu.temp = false;
      gpu = {
        enabled = true;
        vram = false;
      };
      other.battery = false; # handy on a desktop PC
      disks.enabled = true;
    };
  };
}
