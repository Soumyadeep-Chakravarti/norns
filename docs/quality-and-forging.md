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

**Current implementation:** `ItemKind` has `Resource(Resource)`,
`RefinedMaterial(RefinedMaterial)`, and `CraftedItem(CraftedItem)` variants.
Crafted equipment combines an equipment identity with a `MaterialTier`. All
`CraftedItem`s support quality; everything else does not.

The current material model has 18 tiers and the quality model has 22 tiers.
Together they provide 396 material-quality combinations per equipment identity
without defining each combination as a separate item variant.
`ItemKind::supports_quality()` encodes this rule.
Refined Material and Cooked Item types will be introduced with their concrete
content.

## Stack Representations

**Current implementation:** `ItemStack` represents an `ItemKind` and a positive
quantity. For a Crafted Item, it is an unresolved identity/count description,
such as a recipe output, not an owned quality-resolved item. It does not imply
Standard quality.

`CraftedItemStack` represents a `CraftedItem`, an explicit `Quality`, and a
positive quantity. It is the resolved representation for crafted-item rewards
and holdings, regardless of acquisition source. Standard is an actual resolved
tier, not a substitute for missing quality. There are no automatic conversions
from `ItemStack` to `CraftedItemStack`.

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

### Smithing Quality Resolution

**Current implementation:** `SmithingRecipe::output()` describes the item
identity and count as an `ItemStack`. `smith(recipe, quality_roll,
specialization_level)` resolves quality deterministically using `roll_quality()`
and returns a `SmithingOutcome` containing:

- `consumed`: the input `ItemStack`
- `produced`: the resolved `CraftedItemStack`
- Smithing XP and specialization XP

The activity layer supplies the roll and relevant specialization level,
validates eligibility and available inputs, and applies costs and rewards.
Smithing does not generate randomness or mutate player state.

**Current implementation:** The Iron Sword recipe consumes two Iron Ingots.
The smelting activity that produces Iron Ingots from Iron Ore is not implemented
yet, so the complete Iron Ore → Iron Ingot → Iron Sword activity chain remains
unfinished.

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

**Current implementation:** Forge accepts only `CraftedItemStack`, so its input
type restricts the operation to quality-resolved Crafted Items.

## Forge

**Current implementation:** `forge(stack, available_gold)` accepts a
`CraftedItemStack` and `Gold`. The stack supplies a single crafted-item identity,
quality, and available quantity. Forge checks that the quality can be promoted,
at least four items are available, and the gold covers the fee.

On success, `ForgeOutcome` reports:

- `consumed_items`: four
- `produced`: one `CraftedItemStack` at the next quality, preserving the input's
  `CraftedItem` identity
- `gold_spent` and `gold_remaining`

This describes a transaction. The inventory/activity layer subtracts four from
the input stack, adds the promoted item, and applies the gold cost. Forge does
not mutate inventory or return a remaining input stack.

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

Current Iron Sword example:

```text
4 Rare Iron Swords
+ forge fee
        |
        v
1 Epic Iron Sword
```

If the input contains seven Rare Iron Swords, the outcome still reports four
consumed and one Epic Iron Sword produced. Retaining the other three Rare
swords is the inventory/activity layer's responsibility.

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
