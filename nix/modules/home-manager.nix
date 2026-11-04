self:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  inherit (lib)
    mkEnableOption
    mkOption
    mkIf
    mkPackageOption
    types
    ;
  cfg = config.programs.sysprint;
  tomlFormat = pkgs.formats.toml { };

  mkSection =
    name: fields:
    mkOption {
      type = types.submodule {
        options = {
          enabled = mkOption {
            type = types.bool;
            default = true;
            description = "Show the ${name} block";
          };
        }
        // lib.genAttrs fields (
          f:
          mkOption {
            type = types.bool;
            default = true;
            description = "Show `${f}` in the ${name} block";
          }
        );
      };
      default = { };
      description = "The `[${name}]` section";
    };
in
{
  options.programs.sysprint = {
    enable = mkEnableOption "SysPrint";

    package = mkOption {
      type = types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "The sysprint pkg use";
    };

    settings = {
      logo = mkOption {
        type = types.str;
        default = "default";
        description = "Logo to display";
      };
      logo-color = mkOption {
        type = types.enum [
          "default"
          "red"
          "green"
          "blue"
          "yellow"
          "magenta"
          "purple"
          "cyan"
          "white"
          "black"
          "orange"
        ];
        default = "default";
        description = "Logo color";
      };
      custom-logo-path = mkOption {
        type = types.str;
        default = "none";
        example = "/home/user/custom_logo.txt";
        description = "Path to a custom ASCII logo file";
      };
      mini-logo-mode = mkOption {
        type = types.bool;
        default = false;
        description = "Mini logo mode";
      };

      fast-mode = mkOption {
        type = types.bool;
        default = false;
        description = "Fast mode";
      };
      compact-mode = mkOption {
        type = types.bool;
        default = false;
        description = "Compact mode";
      };

      hide-fetch-info = mkOption {
        type = types.bool;
        default = false;
        description = "Hide 'Fetch: SysPrint'";
      };
      config-stronger = mkOption {
        type = types.bool;
        default = false;
      };
      show-sysprint-start-time = mkOption {
        type = types.bool;
        default = true;
      };

      system = mkSection "system" [
        "os-name"
        "kernel"
        "os-version"
        "init"
        "host"
        "user"
        "uptime"
      ];
      cpu = mkSection "cpu" [
        "name"
        "ghz"
        "usage"
        "temp"
        "cores"
        "threads"
        "architecture"
      ];
      gpu = mkSection "gpu" [
        "name"
        "vram"
        "temp"
      ];
      memory = mkSection "memory" [ "ram" ];
      other = mkSection "other" [
        "de"
        "wm"
        "terminal"
        "shell"
        "battery"
        "locale-time"
      ];
      disks = mkSection "disks" [ "list" ];
    };
  };
  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
    xdg.configFile."sysprint/config.toml".source =
      tomlFormat.generate "sysprint-config.tml" cfg.settings;
  };
}
