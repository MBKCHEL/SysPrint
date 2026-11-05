{
  description = "A fast and lightweight ASCII system information fetch tool written in Rust.";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      forAll = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
    in
    {
      packages = forAll (pkgs: rec {
        sysprint = pkgs.callPackage ./nix/package.nix { };
        default = sysprint;
      });

      devShells = forAll (pkgs: {
        default = pkgs.callPackage ./nix/devshell.nix { };
      });

      homeManagerModules.default = import ./nix/modules/home-manager.nix self;
      homeManagerModules.sysprint = self.homeManagerModules.default;
    };
}
