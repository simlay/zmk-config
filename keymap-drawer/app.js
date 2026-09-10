document.querySelectorAll('.key').forEach(key => {
  key.addEventListener('click', () => {
	key.classList.toggle('highlighted');
  });
});

function showLayer(name) {
  document.querySelectorAll('g[class^="layer-"]').forEach(g => {
	g.style.display = 'none';
  });
  document.querySelector(`.layer-${name}`).style.display = 'block';
  document.querySelectorAll('.toolbar button').forEach(b => b.classList.remove('active'));
  event.target.classList.add('active');
}
