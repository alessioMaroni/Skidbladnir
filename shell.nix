{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    gnat
    gnumake
    rustup
    qemu
    OVMF.fd
  ];

  shellHook = ''
    export PATH="${pkgs.gnat}/bin:$PATH"
    export CC=gnatgcc
    export OVMF_PATH="${pkgs.OVMF.fd}/FV/OVMF.fd"
    export RUSTC_WRAPPER=""
  '';
}
