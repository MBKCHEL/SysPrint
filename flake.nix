{
  description = "A fast and lightweight ASCII system information fetch tool written in Rust.";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch-darwin"
        "x86_64-darwin"
      ];
      forAll = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
    in
    {
      packages = forAll (pkgs: rec {
        sysprint = pkgs.rustPlatform.buildRustPackage {
          pname = "sysprint";
          version = (pkgs.lib.importTOML ./Cargo.toml).package.version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          meta = {
            description = "Fast and lightweight ASCII system information fetch tool";
            homepage = "https://github.com/MBKCHEL/SysPrint";
            license = pkgs.lib.licenses.gpl3;
            platforms = pkgs.lib.platforms.unix;
            mainProgram = "sysprint";
          };
        };
        default = sysprint;
      });

      apps = forAll (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.sysprint}/bin/sysprint";
        };
      });

      formatter = forAll (pkgs: pkgs.nixfmt-rfc-style);

      devShells = forAll (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      });
    };
}
