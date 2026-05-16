render:
	keymap parse -z ./config/corne.keymap > corne.yml
	keymap draw corne.yml > corne.svg

watch:
	watchexec -w ./config/ 'make render'
