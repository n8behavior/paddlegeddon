# Paddlegeddon

This project was generated using the [Bevy New
2D](https://github.com/TheBevyFlock/bevy_new_2d) template. Check out the
[documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md)
to get started!

## Core Concept

What starts as innocent Pong transforms into a magical PvP battle arena where
scoring is just the beginning.

## The Evolution Mechanic

- After each goal, the **scored-upon player** receives a power-up
- This creates rubber-band mechanics - losing makes you stronger
- Power-ups stack and compound, creating escalating chaos
- Victory comes not from reaching a score limit, but from vanquishing your
  opponent

## Power-Up Examples

### Paddle Mutations

- Wider/taller paddles
- Split paddles (control multiple)
- Curved paddles that add spin
- Ethereal paddles that phase in/out

### Court Control

- Place obstacles/walls
- Create gravity wells
- Spawn decoy balls
- Teleportation portals

### Ball Manipulation

- Temporary ball control (curve shots)
- Speed bursts/slowdowns
- Invisibility flickers
- Ball splitting into multiple

### Player Transformation

- Evolve into different creatures
- Wizard: spell-casting abilities
- Monster: raw power and size
- Ghost: phase through elements
- Each form has unique abilities

## Environmental Evolution

- Background transforms with each power-up
- Court expands as players grow stronger
- Camera pulls back to accommodate larger play area
- Visual effects intensify (particles, screen shakes, lighting)
- Music evolves from 8-bit bleeps to orchestral battle themes

## Victory Conditions

Traditional scoring becomes secondary. Win by:

- Overwhelming opponent with accumulated powers
- Special finishing moves unlocked at high power levels
- Environmental hazards created by power-ups
- Opponent unable to return increasingly chaotic balls

## Game Flow Example

1. **Classic Start** - Normal Pong for first few points
2. **First Power-Up** - Loser gets wider paddle
3. **Escalation** - More goals = more dramatic powers
4. **Mid-Game Chaos** - Multiple balls, portals, obstacles
5. **End-Game** - Players transformed, arena massive, reality bending
6. **Victory** - One player overwhelmed by sheer chaos

## Technical Considerations

- Power-up system needs careful balancing
- Visual feedback crucial for tracking active powers
- Performance optimization for particle effects and multiple balls
- Save replays of particularly chaotic matches
- Consider asymmetric powers based on playstyle

## Monetization Ideas

- Cosmetic evolution paths (different creature types)
- Arena themes and music packs
- Power-up deck customization
- Tournament modes with standardized power progressions
- Spectator mode with betting on outcomes
