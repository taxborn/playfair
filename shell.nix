# Development shell for playfair. Run `nix-shell` in the repo root, then `just ci`.
#
# Without this, cargo has no linker on NixOS and every build fails with
# ``linker `cc` not found``.
{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  name = "playfair";

  packages = with pkgs; [
    # Provides cc, which cargo shells out to for linking.
    gcc

    rustc
    cargo

    # nixpkgs ships these separately from rustc rather than bundling them the
    # way rustup's default profile does, and `just ci` needs both.
    rustfmt
    clippy

    just
  ];

  shellHook = ''
    echo "playfair dev shell - 'just' lists the available recipes, 'just ci' runs what CI runs."
  '';
}
