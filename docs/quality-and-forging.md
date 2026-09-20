# Quality and Forging

## Quality Eligibility

**Locked design:** Only Crafted Items can have Quality. Crafted Items, Refined
Materials, and Cooked Items are distinct domain categories; "crafted" is not a
synonym for everything produced by a crafting skill.

```text
Items
|-- Raw Resources       (Iron Ore, Logs, Raw Fish)
|-- Refined Materials   (Iron Ingot, Planks, Leather)
|-- Cooked Items        (Cooked Food)
`-- Crafted Items       (Iron Sword, Axe, Bow, Armor)
```

This taxonomy describes design categories; the examples are not all implemented.

The following have no quality:

- raw gathered resources
- refined materials such as planks
- smelted materials
- cooked items

Eligibility is a property of the item category, not its source. A mob drop can
have quality only if it belongs to the Crafted Items category. A mob
dropping resources, refined materials, or cooked items does not give those
items quality.

**Current implementation:** `ItemKind` has `Resource(Resource)` and
`CraftedItem(CraftedItem)` variants. `CraftedItem::IronSword` is the first crafted
identity, and `ItemKind::supports_quality()` is true only for `CraftedItem`.
Refined Material and Cooked Item types will be introduced with their concrete
content. This eligibility check does not attach quality to the universal stack
or integrate quality rolling into Smithing yet.

## Resource Quality

**Locked design:** Raw gathered resources do not have item quality.

Mining and other resource-gathering activities produce resource stacks with an
item identity and quantity. Quality is reserved for the Crafted Items category.

For example:

```text
Copper Ore x50
```

is a resource stack, not a `Standard Copper Ore` stack.

**Current implementation:** `ItemStack` contains an `ItemKind` and a positive
quantity. Mining returns a resource stack alongside mining and specialization
XP, without a quality roll.

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

This rule applies to quality rolling for quality-bearing items, not to raw
resource gathering. Forge upgrades do not use specialization or quality rolls.

**Balance placeholder:** The current luck coefficient, including the `750`
basis-point increment, requires simulation and balancing.

## Forge Eligibility

**Locked design:** The Forge is a separate economic system that operates only
on eligible quality-bearing items, independently of gathering, crafting,
specialization, and the quality-roll operation.

Raw resources are not forgeable merely because they can be represented as an
`ItemStack`.

The representation of quality for `CraftedItem::IronSword` remains to be
designed. Its category establishes eligibility, not a stored quality value.

## Forge

**Current implementation:** The core Forge calculation accepts a quality,
available item count, and gold. It checks the count and fee, then reports the
next quality and costs. It does not yet model item identity or enforce category
eligibility.

**Locked design:** Four identical eligible items of the same quality plus gold
produce one of the same item at the next quality:

```text
4 identical items
+ same quality
+ gold
        |
        v
1 identical item
at the next quality
```

Illustrative future equipment example:

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
