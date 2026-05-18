render:
	keymap parse -z ./config/corne.keymap > keymap-drawer/corne.yml
	keymap draw corne.yml > keymap-drawer/corne.svg

watch:
	watchexec -w ./config/ 'make render'
