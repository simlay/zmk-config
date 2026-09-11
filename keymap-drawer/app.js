// Track pressed keys for visual feedback
const pressedKeys = new Set();

// Find all keys by their label text
function findKeysByLabel(label) {
  const allKeys = document.querySelectorAll('.key');
  const matches = [];
  for (const key of allKeys) {
	const textEl = key.querySelector('text.tap');
	if (textEl && textEl.textContent === label) {
	  matches.push(key);
	}
  }
  return matches;
}

// Add keyboard event listeners
document.addEventListener('keydown', (e) => {
  if (e.ctrlKey || e.altKey || e.metaKey) return;

  const key = e.key;
  if (key === undefined || key.length > 1) return;

  let label = key;
  if (key.length === 1) {
	label = key.toUpperCase();
  }

  const keyGroups = findKeysByLabel(label);
  keyGroups.forEach(keyGroup => {
	const keyRect = keyGroup.querySelector('rect.key');
	if (keyRect && !pressedKeys.has(key)) {
	  keyRect.style.fill = '#ff4444';
	  pressedKeys.add(key);
	}
  });
});

document.addEventListener('keyup', (e) => {
  if (e.ctrlKey || e.altKey || e.metaKey) return;

  const key = e.key;
  if (key === undefined || key.length > 1) return;

  let label = key.toUpperCase();
  const keyGroups = findKeysByLabel(label);
  keyGroups.forEach(keyGroup => {
	const keyRect = keyGroup.querySelector('rect.key');
	if (keyRect) {
	  keyRect.style.fill = '';
	}
  });
  pressedKeys.delete(key);
});
