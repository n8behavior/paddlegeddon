# Paddlegeddon

This project was generated using the [Bevy New
2D](https://github.com/TheBevyFlock/bevy_new_2d) template. Check out the
[documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md)
to get started!

## Core Concept

What starts as innocent Pong transforms into a magical PvP battle arena where
scoring is just the beginning.

## The Evolution Mechanic

- **Rally Power**: Each hit in a rally earns escalating charge points (1, 2,
  3... or Fibonacci sequence)
- **Charge Economy**: Spend points to activate abilities OR save to unlock new
  ones
- **Evolution Phases**: At combined scores (5, 10, 20, 30, 50), both players
  enter ability shop
- **Chaos Events**: Timed environmental changes affect both players equally
- **Strategic Depth**: Balance immediate power vs. long-term investment
- Victory comes through overwhelming chaos and strategic ability use

## Unlockable Abilities

### Tier 1 (10-20 charge points to unlock)

- **Power Shot** (3 points to activate) - Launches ball at double speed
- **Quick Dash** (2 points) - Instant paddle repositioning
- **Spin Shot** (4 points) - Adds heavy curve to returns
- **Paddle Extend** (5 points) - Temporarily doubles paddle size

### Tier 2 (30-50 charge points to unlock)

- **Multi-Ball** (8 points) - Splits ball into three on next hit, each moving
  on different vector
- **Phase Shift** (10 points) - Paddle becomes ethereal, ball passes through
- **Gravity Well** (12 points) - Creates attraction point on court
- **Portal Pair** (15 points) - Place entrance/exit teleportation gates

### Tier 3 (100+ charge points to unlock)

- **Reality Warp** (20 points) - Reverses controls for opponent
- **Time Dilation** (25 points) - Slows time in half the court
- **Chaos Storm** (30 points) - Spawns multiple hazards and effects
- **Form Evolution** (50 points) - Transform into powerful creature

### Ability Notes

- Each ability has cooldowns to prevent spam
- Higher tiers unlock at evolution thresholds
- Strategic choice: frequent cheap abilities vs. saving for ultimates

## Environmental Evolution

- Background transforms with each power-up
- Court expands as players grow stronger
- Camera pulls back to accommodate larger play area
- Visual effects intensify (particles, screen shakes, lighting)
- Music evolves from 8-bit bleeps to orchestral battle themes

## Victory Conditions

Traditional scoring remains but becomes secondary to chaos escalation. Win by:

- Strategic ability combos that overwhelm your opponent
- Ultimate abilities that dramatically shift the game state
- Forcing errors through well-timed ability usage
- Surviving the escalating chaos better than your opponent
- Natural match ending when chaos reaches critical mass

## Game Flow Example

1. **Classic Start** - Normal Pong, players earn charge points from rallies
2. **First Evolution** (5 combined score) - Ability shop opens, +10 bonus
   points each
3. **Early Strategy** - Players unlock Tier 1 abilities, testing combos
4. **Arena Expansion** (10 score) - Court grows, first chaos event triggers
5. **Mid-Game** - Tier 2 abilities available, multiple balls and portals appear
6. **Chaos Awakening** (20 score) - Environmental hazards, reality distortions
7. **End-Game** - Ultimate abilities unleashed, arena transforms dramatically
8. **Victory** - One player succumbs to overwhelming chaos or devastating combo

## Technical Considerations

- Charge point economy needs careful balancing (costs vs. earning rates)
- Rally detection system for escalating point values
- UI for ability shop during evolution phases
- Visual feedback for charge points, cooldowns, and active abilities
- Configurable number series for rally progression (linear, Fibonacci, etc.)
- Performance optimization for particle effects and multiple balls
- Save replays of particularly chaotic matches
- Chaos event timing and randomization system

## Monetization Ideas

- Cosmetic ability effects and animations
- Alternative number series for rally progression
- Arena themes and music packs
- Custom ability unlock trees
- Tournament modes with standardized charge economies
- Spectator mode with betting on outcomes
- Ability skin packs (same function, different visuals)
