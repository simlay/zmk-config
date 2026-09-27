// Sample texts generated from an LLM which are paraphrased to be things from enders game.
const sampleTexts = [
  {
    name: "The Battle School",
    text: `The Battle School was not like any school he had ever heard about in his old life. There were no classrooms, no teachers in the traditional sense, no desks or chairs. Instead, there were games, real games played in zero gravity where the rules changed constantly and the only way to win was to think in ways no one had thought before. Ender Wiggin was six years old when he arrived, the youngest student in his entire class, but he learned faster than anyone else. He watched his teammates, studied his opponents, and adapted. The dragon always found a way.`
  },
  {
    name: "Command School",
    text: `In Command School, Ender was given simulations that seemed impossible. Thousands of ships against thousands of ships, and he had to command them all. He won every battle, but the cost was enormous. Each victory left him hollow, because he knew the enemy was real. The simulations were not simulations at all. They were the final preparations for a war that would decide the fate of all humanity. Only then did he understand the true meaning of strategy and sacrifice.`
  },
  {
    name: "The Hive Queen",
    text: `When Ender finally reached the Formics, he found not monsters but a dying civilization. The Hive Queen, alone and broken, offered him the last gift of her species. Ender wept for what he had done and for what he had been made to do. He carried the weight of billions of lives on his shoulders, but in the end, he chose mercy over destruction. He would find a new home for the surviving Formics, a world where they could rebuild and thrive. He was no longer a weapon but a guardian.`
  },
  {
    name: "On Friendship",
    text: `Friendship is not something you learn in school. It is something you learn in the moments when you realize that someone understands you, someone who sees through your armor and your games and your strategies to find the person underneath. Valentine was the only person who truly knew Ender Wiggin. She loved him not for what he could do but for who he was. And in a universe of wars and battles, that love was the most powerful force of all.`
  },
  {
    name: "The End and The Beginning",
    text: `The end is the beginning. Ender Wiggin destroyed an entire species, but in doing so, he saved humanity from destruction. He carried the guilt for the rest of his life, but he also carried hope. The monolith he sent back to the Formic world contained the embryo of the Hive Queen, a chance for her species to survive and perhaps one day to forgive. The universe is vast and full of unknowns, but as long as there are those who can see the other side, there is always hope for redemption and understanding.`
  },
  {
    name: "The Monitor",
    text: `The Monitor had watched Ender grow from the moment he was born, feeding information back to the Battle Commanders who trained him for a war he never knew existed. Every game, every lesson, every friendship was orchestrated to forge him into the perfect commander. When Ender finally learned the truth, he understood that the Monitor was not his enemy but his guardian, bound by its own programming to protect both humanity and the alien species it had been sent to study.`
  },
  {
    name: "The Ibo Children",
    text: `On the bus to Battle School, Ender watched the Ibo children who could read each other's minds. They spoke in a secret language only they understood, a gift that made them both powerful and isolated. Ender realized that true leadership meant understanding others, even those who seemed completely different from yourself. He made a silent promise to never let anyone feel as alone as he had felt, to bridge the gaps between people rather than widen them.`
  },
  {
    name: "The Seed Queen",
    text: `Ender became the Speaker for the Dead, carrying the last thoughts of a dying species across the stars. He searched for generations for the Seed Queen, the last of her kind, until he finally found her hidden in a remote world. In that moment of reunion, Ender understood that every life has value, every species deserves to be remembered, and the stories we tell about the dead are what keep them alive in the hearts of the living.`
  },
  {
    name: "The Colonel John Paul",
    text: `Ender named his battle groups after the people who had loved him most, carrying their memories into every simulation and every strategy session. Colonel John Paul had been his first teacher, showing him that the best way to win was to understand your enemy better than they understood themselves. Those early lessons in empathy and strategy became the foundation of everything Ender would later achieve, proving that the seeds of greatness are planted in childhood.`
  },
  {
    name: "The Planet Formic IV",
    text: `When Ender finally arrived at the Formic home world, he found it empty, scorched by the atomic bombs he had thought were merely simulations. The silence of the dead planet weighed on him heavier than any victory could compensate. He spent days walking among the ruins, trying to understand what he had done and whether forgiveness was possible across the vast gulf between species. In the end, he chose to believe that understanding could heal what destruction had broken.`
  }
];
