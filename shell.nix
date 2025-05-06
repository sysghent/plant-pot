{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    picotool
    elf2uf2-rs
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];
}
