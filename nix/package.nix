{
  lib,
  rustPlatform,
  ...
}:

rustPlatform.buildRustPackage {
  pname = "sysprint";
  version = (lib.importTOML ../Cargo.toml).package.version;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../assets
      ../Cargo.toml
      ../Cargo.lock
    ];
  };
  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    description = "Fast and lightweight ASCII system information fetch tool";
    homepage = "https://github.com/MBKCHEL/SysPrint";
    license = lib.licenses.gpl3;
    platforms = lib.platforms.unix;
    mainProgram = "sysprint";
  };
}
