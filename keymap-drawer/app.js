function updateKeys(e, fill) {
  if (e.ctrlKey || e.altKey || e.metaKey) return;
  const key = e.key;

  if (key === undefined || key.length > 1) return;

  let label = key;
  if (key.length === 1) {
    label = key.toUpperCase();
  }

  const rects = Array.from(document.querySelectorAll('.key'))
    .filter(key => key.querySelector('text.tap')?.textContent === label)
    .map(key => key.querySelector('rect.key'));

  rects.forEach(rect => {
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
