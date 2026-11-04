{
  mkShell,
  cargo,
  rustc,
  rust-analyzer,
  clippy,
  rustfmt,
}:

mkShell {
  packages = [
    cargo
    rustc
    rust-analyzer
    clippy
    rustfmt
  ];
}
