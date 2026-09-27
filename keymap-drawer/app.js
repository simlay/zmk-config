// ─── Key highlight helpers (existing functionality) ───

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

// ─── Typing Test & Training Tool ───

const TYPING_WINDOW_MS = 3 * 60 * 1000; // 3 minutes in ms

function initTypingTest() {
  const container = document.getElementById('typing-test');
  if (!container) return;

  const textSelect = container.querySelector('.text-select');
  const textDisplay = container.querySelector('.text-display');
  const wpmDisplay = container.querySelector('.wpm-display');
  const input = container.querySelector('.typing-input');
  const resetBtn = container.querySelector('.reset-btn');

  // Populate text selector
  sampleTexts.forEach((st, i) => {
    const opt = document.createElement('option');
    opt.value = i;
    opt.textContent = st.name;
    textSelect.appendChild(opt);
  });

  let currentText = '';
  let position = 0; // current character position
  let charStates = []; // 'pending' | 'confirmed' | 'wrong'
  let startTime = null;
  let wordTimes = []; // timestamps of completed words
  let isFinished = false;

  function initCharStates() {
    charStates = new Array(currentText.length).fill('pending');
  }

  function renderText() {
    textDisplay.innerHTML = '';
    for (let i = 0; i < currentText.length; i++) {
      const span = document.createElement('span');
      span.className = 'char';
      span.textContent = currentText[i];

      if (charStates[i] === 'confirmed') {
        span.classList.add('correct');
      } else if (charStates[i] === 'wrong') {
        span.classList.add('wrong');
      } else if (i === position) {
        span.classList.add('current');
      } else {
        span.classList.add('pending');
      }
      textDisplay.appendChild(span);
    }

    // Scroll current character into view
    const currentSpan = textDisplay.querySelector('.current');
    if (currentSpan) {
      currentSpan.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
    }
  }

  function updateWPM() {
    const now = Date.now();
    // Filter word times to the rolling window
    wordTimes = wordTimes.filter(t => now - t < TYPING_WINDOW_MS);

    if (wordTimes.length === 0) {
      wpmDisplay.textContent = 'WPM: 0';
      return;
    }

    const elapsedMin = (now - wordTimes[0]) / 60000;
    if (elapsedMin <= 0) {
      wpmDisplay.textContent = 'WPM: 0';
      return;
    }

    const wpm = Math.round(wordTimes.length / elapsedMin);
    wpmDisplay.textContent = `WPM: ${wpm}`;
  }

  function advancePosition() {
    // Track word completion when passing through a space
    if (currentText[position] === ' ') {
      wordTimes.push(Date.now());
    }

    position++;
    if (position >= currentText.length) {
      // End of text
      isFinished = true;
    }
    updateWPM();
  }

  function handleKeyPress(key) {
    if (isFinished || position >= currentText.length) return;

    if (!startTime) {
      startTime = Date.now();
    }

    const expected = currentText[position];

    if (key === expected) {
      charStates[position] = 'confirmed';
      advancePosition();
    } else {
      // Wrong key - mark as wrong but still advance
      charStates[position] = 'wrong';
      advancePosition();
    }

    renderText();
  }

  function handleBackspace() {
    if (position <= 0) return;

    // Go back to previous position
    position--;

    // Remove the word time if going back over a space
    if (currentText[position] === ' ' && wordTimes.length > 0) {
      wordTimes.pop();
    }

    // Reset the character state to pending
    charStates[position] = 'pending';

    renderText();
  }

  function startNewText() {
    const idx = Math.floor(Math.random() * sampleTexts.length);
    currentText = sampleTexts[idx].text;
    position = 0;
    startTime = null;
    wordTimes = [];
    isFinished = false;
    initCharStates();
    renderText();
    updateWPM();
    textSelect.value = idx;
    input.focus();
  }

  // Use keydown to capture individual keystrokes
  input.addEventListener('keydown', (e) => {
    if (e.key === 'Backspace') {
      e.preventDefault();
      handleBackspace();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      startNewText();
    } else if (e.key.length === 1) {
      // Regular character key - let it pass through for keyboard highlighting
      handleKeyPress(e.key);
      return;
    }
  });

  resetBtn.addEventListener('click', startNewText);
  textSelect.addEventListener('change', startNewText);

  // Start with first text
  startNewText();
}
