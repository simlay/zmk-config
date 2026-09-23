render:
	keymap parse -z ./config/corne.keymap > keymap-drawer/corne.yml
	keymap draw keymap-drawer/corne.yml > keymap-drawer/corne.svg
	./build-interactive.sh

watch:
	watchexec -w ./config/ 'make render'


get-firmware:
	mkdir -p firmware/
	rm firmware/corne_right-nice_nano_v2-zmk.uf2 firmware/corne_left-nice_nano_v2-zmk.uf2
	gh run download $(shell gh run list --workflow 'Build ZMK firmware' -b main -L 1 -s completed  --json 'databaseId' --jq '.[].databaseId')
