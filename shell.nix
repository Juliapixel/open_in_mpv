{
  pkgs,
  ...
}:
pkgs.mkShell {
  packages = with pkgs; [
    web-ext
    fenix.stable.toolchain
  ];
}
