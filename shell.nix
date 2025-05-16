{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    clang_19
    cargo-binutils # For `cargo objdump` and `cargo size`
    elf2uf2-rs # If your device uses UF2 files for flashing.
    nushell
    picotool # In case you want to use offical Raspberry Pi tools
    probe-rs-tools # For breakpoints in VS Code you need version 0.28.0 or higher
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
    export NIX_ENFORCE_PURITY=0
    ${pkgs.nushell}/bin/nu
  '';
}
