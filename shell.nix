{
  pkgs ? import <nixpkgs> { },

}:

pkgs.mkShell {

  buildInputs = with pkgs; [
    clang_19
    cargo-binutils # For `cargo objdump` and `cargo size`
    elf2uf2-rs # If your device uses UF2 files for flashing.
    picotool # In case you want to use offical Raspberry Pi tools
    rustup
    tio # For reading serial output
    openocd
    gdb
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
  '';
}
