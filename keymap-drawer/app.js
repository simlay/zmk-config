function findKeysByLabel(label) {
  return Array.from(document.querySelectorAll('.key'))
    .filter(key => key.querySelector('text.tap')?.textContent === label)
    .map(key => key.querySelector('rect.key'));
}

function updateKeys(e, fill) {
  const key = e.key;

  // Special keys (strings = direct labels, functions = compute from e)
  const specialKeyMap = {
    ' ': 'SPACE',
    'Backspace': 'BSPC',
    'Tab': 'TAB',
    'Enter': 'RET',
    'Escape': 'ESC',
    'ArrowLeft': 'LEFT',
    'ArrowUp': 'UP',
    'ArrowDown': 'DOWN',
    'ArrowRight': 'RIGHT',
    'Control': (e) => `${e.code.includes('Left') ? 'L' : 'R'}CTRL`,
    'Shift': (e) => `${e.code.includes('Left') ? 'L' : 'R'}SHFT`,
    'Meta': (e) => `${e.code.includes('Left') ? 'L' : 'R'}GUI`
  };

  if (specialKeyMap[key]) {
    const label = typeof specialKeyMap[key] === 'function'
      ? specialKeyMap[key](e)
      : specialKeyMap[key];
    findKeysByLabel(label).forEach(rect => {
      rect.style.fill = fill;
    });
    return;
  }

  if (key === undefined) return;

  let label = key;
  if (key.length === 1) {
    label = key.toUpperCase();
  }

  findKeysByLabel(label).forEach(rect => {
    rect.style.fill = fill;
  });
}

// Add keyboard event listeners
document.addEventListener('keydown', (e) => {
  updateKeys(e, '#ff4444');
});

document.addEventListener('keyup', (e) => {
  updateKeys(e, '');
});
