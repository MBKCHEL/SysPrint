{
  lib,
  stdenv,
  rustPlatform,
  makeWrapper,
  hwdata,
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

  nativeBuildInputs = lib.optionals stdenv.hostPlatform.isLinux [ makeWrapper ];

  postInstall = lib.optionalString stdenv.hostPlatform.isLinux ''
    wrapProgram $out/bin/sysprint \
      --prefix XDG_DATA_DIRS : ${hwdata}/share
  '';

  meta = {
    description = "Fast and lightweight ASCII system information fetch tool";
    homepage = "https://github.com/MBKCHEL/SysPrint";
    license = lib.licenses.gpl3;
    platforms = lib.platforms.unix;
    mainProgram = "sysprint";
  };
}
