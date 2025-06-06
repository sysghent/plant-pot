{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {

  buildInputs = with pkgs; [
    cargo-binutils # For `cargo objdump` and `cargo size`
    gdb-multitarget
    minicom
    openocd
    picotool
    probe-rs-tools
    rustup
    tio # For reading serial output
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
  '';
}
