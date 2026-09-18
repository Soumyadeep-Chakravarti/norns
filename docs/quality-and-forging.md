# Quality and Forging

## Quality Ladder

**Current implementation:** Norns has a 22-tier quality ladder.

| Tier | Quality | Base rarity |
| ---: | --- | ---: |
| 0 | Standard | guaranteed baseline |
| 1 | Common | 1 / 2 |
| 2 | Uncommon | 1 / 5 |
| 3 | Rare | 1 / 10 |
| 4 | Epic | 1 / 17 |
| 5 | Legendary | 1 / 33 |
| 6 | Mythic | 1 / 67 |
| 7 | Ancient | 1 / 143 |
| 8 | Relic | 1 / 333 |
| 9 | Runebound | 1 / 1,000 |
| 10 | Cursed | 1 / 2,000 |
| 11 | Eldritch | 1 / 5,000 |
| 12 | Forgotten | 1 / 10,000 |
| 13 | Fatebound | 1 / 20,000 |
| 14 | Hel-Forged | 1 / 50,000 |
| 15 | Jotunnforged | 1 / 100,000 |
| 16 | Voidborn | 1 / 200,000 |
| 17 | Einherjar | 1 / 500,000 |
| 18 | Aesir-Touched | 1 / 667,000 |
| 19 | Worldforged | 1 / 833,000 |
| 20 | Norn-Touched | 1 / 1,000,000 |
| 21 | Primordial | 1 / 2,000,000 |

The implementation preserves Norse spellings in display names: `Jotunnforged`
is displayed as `Jötunnforged`, and `Aesir-Touched` as `Æsir-Touched`.

**Balance placeholder:** Base rarity values are quality thresholds prior to
specialization and future modifiers.

## Quality Rolls

**Current implementation:** The quality roller accepts values in this range:

```text
0 .. 2,000,000
```

`norns-core` deterministically calculates the resulting quality from the roll
and specialization level. This separation avoids an RNG dependency in the core.

**Locked design:** The server will eventually generate the random roll.

## Specialization Luck

**Current implementation:** Luck is an integer multiplier in basis points:

```text
10,000 = 1.00x
```

Higher specialization improves quality chances using diminishing growth.

**Balance placeholder:** The current luck coefficient, including the `750`
basis-point increment, requires simulation and balancing.

## Forge

**Current implementation:** The Forge deterministically upgrades four items of
the same quality to one item of the next quality, provided the player has the
required gold.

```text
4 identical items
+ same quality
+ gold
        |
        v
1 identical item
at the next quality
```

Example:

```text
4 Rare Iron Swords
+ forge fee
        |
        v
1 Epic Iron Sword
```

Forging is guaranteed and does not use RNG. Primordial is final and cannot be
forged further.

**Balance placeholder:** Forge prices currently start at `100` gold and double
per resulting tier. These values are provisional.

## Economic Purpose

**Locked design:** The Forge is both an item and gold sink. Four items become
one, so deterministic upgrades consume exponentially more base items:

```text
1 tier increase  = 4 base items
2 tier increases = 16 base items
3 tier increases = 64 base items
4 tier increases = 256 base items
```

## Two Paths to Quality

**Locked design:** Exceptional items should be available through both natural
quality rolls and deterministic forging.

```text
               High-quality item
                /             \
               /               \
      Natural quality roll      Forge
        luck / mastery       items + gold
```

Natural rolls provide rare jackpot outcomes. Forging provides deterministic
progression at substantial economic cost. Together they preserve the value of
rare natural drops without making quality progression entirely luck-dependent.
