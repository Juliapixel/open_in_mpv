# Open in MPV

Adds a button in your browser's context menu to open either the hovered-over image/link or the current webpage on [MPV](https://mpv.io/).

only works on linux currently.

## Usage

1. install mpv
2. clone the repo
3. run `cargo build -r` in order to build the handler program
4. run `sudo ./target/release/protocol_handler start`, this will add the .desktop entry to your `/usr/share/applications/` folder, associating all `mpv://` urls to the handler.
5. if
