# Open in MPV

Adds a button in your browser's context menu to open either the hovered-over image/link or the current webpage on [MPV](https://mpv.io/).

only works on linux currently.

## Usage

1. install mpv.
2. clone the repo.
3. run `cargo build -r` in order to build the handler program.
4. run `sudo ./target/release/protocol_handler start`, this will add the .desktop entry to your `/usr/share/applications` folder, associating all `mpv://` urls to the handler.
5. in order to install the extension from the `./extension` folder you will need to use either firefox nightly or firefox developer, by zipping the contents of the `./extension` folder, going to `about:addons` and installing the zipped extension from file. otherwise, you will have to go to `about:debugging#/runtime/this-firefox` and load the addon temporarily.


😎
