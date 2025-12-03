{
  fetchFromGitHub,
  rustPlatform,
}:
let
  props = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;
  name = props.name;
  version = props.version;
in
rustPlatform.buildRustPackage {
  pname = name;
  version = version;

  cargoLock.lockFile = ./Cargo.lock;

  src = fetchFromGitHub {
    owner = "Juliapixel";
    repo = "open_in_mpv";
    tag = "v1.0.3";
    sha256 = "sha256-pCmwFzpU2qO+a5S7uJLsH5ISTjXmsXrIbPtOiitNfJg=";
  };

  preInstall = ''
    mkdir -p $out/share/applications
    echo "[Desktop Entry]
    Type=Application
    NoDisplay=true
    Name=MPV URL handler
    Exec=''${out}/bin/open_in_mpv %U
    StartupNotify=false
    Terminal=false
    MimeType=x-scheme-handler/mpv;" > $out/share/applications/open_in_mpv.desktop
  '';
}
