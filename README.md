# Open in MPV

Adds a button in your browser's context menu to open either the hovered-over image/link or the current webpage on [MPV](https://mpv.io/).

only works on linux currently.

## Usage

### Registering the handler and installing the extension
1. install mpv.
2. clone the repo.
3. run `cargo build -r` in order to build the handler program.
4. run `sudo ./target/release/protocol_handler register`, this will add the .desktop entry to your `/usr/share/applications` folder, associating all `mpv://` urls to the handler.
5. in order to install the extension from the `./extension` folder you will need to use either firefox nightly or firefox developer, by zipping the contents of the `./extension` folder, going to `about:addons` and installing the zipped extension from file. otherwise, you can download one of the [releases](https://github.com/Juliapixel/open_in_mpv/releases) and install that.

### Uninstalling
1. run `sudo .target/release/protocol_handler remove`.
2. remove the extension if you installed it to your browser.
3. delete the downloaded files.

😎
