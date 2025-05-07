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
    export NIX_ENFORCE_PURITY=0
    ${pkgs.nushell}/bin/nu
  '';
}
