render:
	keymap parse -z ./config/corne.keymap > keymap-drawer/corne.yml
	keymap draw keymap-drawer/corne.yml > keymap-drawer/corne.svg
	./build-interactive.sh

watch:
	watchexec -w ./config/ 'make render'
