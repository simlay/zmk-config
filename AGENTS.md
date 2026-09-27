This is a zmk config repo for a corne keyboard. The keymap is in
config/corne.keymap

If there exists a directory `zmk`, that is the git checkout of the `zmk`
repository and `zmk/docs` are the docs for zmk.

The primary use of this keyboard layout is vim, tmux and bash/zsh on macOS.

One of the goals is to support use with one hand on either half of the keyboard
layout.

There is a interactive static webpage that's built from the `make render` rule.
It exists in `./keymap-drawer`. The files of that static page are
`keymap-drawer/app.js`, `keymap-drawer/index.base.html` and
`keymap-drawer/corne.svg` which are bundled together to make
`keymap-drawer/index.html`.
