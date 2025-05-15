{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo-binutils
    elf2uf2-rs
    nushell
    picotool
    probe-rs
    rustup
    tio
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  shellHook = ''
    export NIX_ENFORCE_PURITY=0
    ${pkgs.nushell}/bin/nu
  '';
}
