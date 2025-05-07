{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustup
    picotool
    elf2uf2-rs
    nushell
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  shellHook = ''
    ${pkgs.nushell}/bin/nu
  '';
}
