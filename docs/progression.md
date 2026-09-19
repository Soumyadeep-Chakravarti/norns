# Progression

## Infinite Levels

**Locked design:** Skills do not have a hard maximum level.

```text
Levels 1-100    content progression
Levels 101+     mastery progression
```

Most ordinary content requirements should remain within the first 100 levels.
Progression beyond level 100 is for long-term mastery, specialization,
efficiency, prestige, and other diminishing benefits.

## Experience Curve

**Current implementation:**

```text
x = level - 1

XP(level) = 100x^2 + 5x^3
```

The implementation uses integer arithmetic. XP-to-level conversion uses binary
search rather than analytically inverting the cubic.

**Balance placeholder:** The curve is a mechanical foundation, but its
coefficients may change following balancing work.

## Saturation

**Current implementation:** Experience uses `u64`, and accumulation uses
saturating arithmetic so overflow cannot wrap back to a small value.

## Specialization

**Current implementation:** Specialization progression is represented
independently from broad skill progression.

**Locked design:** Broad skills contain narrower specialization progression.

```text
Mining
|-- Stone
|-- Copper
|-- Tin
`-- Iron

Smithing
|-- Sword
|-- Axe
`-- Armor

Melee
|-- Sword
|-- Axe
`-- Spear
```

Crafting an axe and fighting with an axe are separate forms of mastery. A
player may therefore be an exceptional axe smith without being an exceptional
axe fighter.

## Purpose

**Locked design:** Specialization supports economic and gameplay identities.
Instead of every high-level player becoming equally good at everything, players
can become known as specialist miners, weapon smiths, fishermen, cooks, or
weapon users. This is especially important to the planned player-driven
economy.

## Specialization and Quality

**Current implementation:** Quality luck uses integer basis points, where
`10,000` represents `1.00x`, and specialization increases it with diminishing
growth.

**Locked design:** Quality luck is relevant only to quality-bearing item
categories. Raw gathered resources have no quality; resource specialization
does not introduce quality rolls into Mining. Forge is a separate deterministic
economic mechanic and does not use specialization luck.

**Balance placeholder:** The current coefficient and progression behavior need
simulation and balancing. Infinite specialization must not make the highest
qualities trivial.
