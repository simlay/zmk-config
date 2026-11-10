This is a zmk config repo for a corne keyboard. The keymap is in
config/corne.keymap

If there exists a directory `zmk`, that is the git checkout of the `zmk`
repository and `zmk/docs` are the docs for zmk.

The primary use of this keyboard layout is vim, tmux and bash/zsh on macOS.

One of the goals is to support use with one hand on either half of the keyboard
layout.

There is a interactive static webpage that's built from the `make render` rule.
It exists in `./keymap-drawer`. The files of that are a rust webassembly
project that use `trunk`.
`keymap-drawer/corne.svg` is bundled into the rust via the `include_str!` macro.
`keymap-drawer/index.html` is the input HTML. Trunk takes it and generates
stuff into `keymap-drawer/dist/`
