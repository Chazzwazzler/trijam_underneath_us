# TODO

- [X] Fix rendering
  - [X] Fix pixel perfect scaling
  - [X] Set minimum window size
- [X] Code cleanup
  - [X] Organize constants in a dedicated constants file
  - [X] Move rendering code to a separate file
- [ ] Player movement
  - [X] Create an input buffer with timers (not frame-based; should be stored in the database)
  - [X] Left/right movement
  - [ ] Jumping
  - [ ] Diving
- [ ] Player animations
  - [ ] Jump animation
  - [ ] Dive animation
- [ ] Bat enemy
  - [ ] Spawn bats
  - [ ] Make bats move towards player
  - [ ] Add animation
- [ ] Armored enemy
  - [ ] Spawn armored enemies
  - [ ] Make them move side to side
  - [ ] Add animation
- [ ] Combat
  - [ ] Add player health, constantly decreasing (health bar above player)
  - [ ] Take damage on collision with bat (damage buffer..?)
  - [ ] Parry bat (use the input buffer)
  - [ ] Kill armored enemies by diving
  - [ ] Spawn orbs on enemy death
  - [ ] Collect orbs and refill player health
- [ ] Main menu
  - [ ] Render title/art
  - [ ] Press any button to start
- [ ] Game over
  - [ ] Display score
  - [ ] Press any button to restart (or something)
- [ ] Sfx
  - [ ] Start game sfx
  - [ ] Game over sfx
  - [ ] Jump sfx
  - [ ] Bat flap sfx
  - [ ] Kill enemy sfx
  - [ ] Collect soul-orb sfx