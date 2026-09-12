#!/bin/bash

awk 'BEGIN { while ((getline line < "keymap-drawer/corne.svg") > 0) { if (svg != "") svg = svg "\n"; svg = svg line } } { pos = index($0, "SVG_PLACEHOLDER"); if (pos > 0) { print substr($0, 1, pos-1) svg
 substr($0, pos + length("SVG_PLACEHOLDER")); } else { print } }' keymap-drawer/index.base.html > keymap-drawer/index.html
