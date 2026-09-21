# Minimum Content

This document translates the Realm Idle reference catalog into Norns content
requirements. The reference supplies the breadth and progression shape; Norns
uses its own names, lore, economy, and item rules.

Source reference: <https://realmidle.com/>

Values copied from the reference are balance inputs, not locked mechanics. A
number is not implementation-ready until its input costs, duration, failure
behavior, and player-state transaction are specified.

## Content Boundary

The minimum content target includes:

- 10 combat zones, including the Celestial Realm.
- 15 player skills: the current 14 skills plus Divinity and Thieving.
- Gathering, production, combat, equipment, consumables, drops, and currency.
- Activities with level requirements, XP rewards, resource outputs, and
  progression dependencies.
- A complete path from early gathering to endgame equipment and encounters.

The reference's ninth zone, Stormcaller, is marked "Coming Soon" on the site,
but it is included here because it bridges the eighth zone and the Celestial
Realm in the published lore.

## Skill Mapping

| Reference concept | Norns representation | Status |
| --- | --- | --- |
| Attack, Strength, Defense | Melee skill and specializations | Existing taxonomy |
| Ranged | Ranged skill | Existing taxonomy |
| Magic | Seidr skill | Existing taxonomy |
| Woodcutting | Woodcutting | Existing taxonomy |
| Mining | Mining | Implemented vertical slice |
| Fishing | Fishing | Existing taxonomy |
| Foraging | Foraging | Existing taxonomy |
| Hunting | Hunting | Existing taxonomy |
| Smithing | Smithing | Initial recipe implemented |
| Crafting | Woodworking and Leatherworking | Split by material family |
| Cooking | Cooking | Existing taxonomy |
| Alchemy | Alchemy | Existing taxonomy |
| Arcane Arts | Runecrafting and Seidr production | Split by output |
| Divinity | Divinity | New skill required |
| Thieving | Thieving | New skill required |

Combat stats such as Attack, Strength, and Defense should not become three
parallel top-level skills unless later design work shows that they need
independent progression. They are better represented as Melee specializations
or combat attributes for the first implementation.

## Zones

| # | Zone | Minimum level | Entry cost | Boss |
| ---: | --- | ---: | ---: | --- |
| 1 | Verdant Meadows | 1 | Free | Goblin Chieftain |
| 2 | Darkwood Forest | 38 | 5,000 gold | Moss Giant Elder |
| 3 | Ancient Dungeon | 65 | 25,000 gold | Lich King |
| 4 | Shadowlands | 78 | 100,000 gold | Voidspawn |
| 5 | Molten Peak | 91 | 350,000 gold | Infernal Lord |
| 6 | Frozen Wastes | 101 | 1,000,000 gold | Frozen Overlord |
| 7 | Abyssal Depths | 130 | 5,000,000 gold | Drowned Colossus |
| 8 | Drowned Ruins | 145 | 15,000,000 gold | Relic Wraith |
| 9 | Stormcaller | Unspecified | Unspecified | Vaelthrax |
| 10 | Celestial Realm | Unspecified | Unspecified | Celestial threat |

Each zone needs a data-driven definition for its required level, unlock cost,
encounter table, boss, rewards, and lore. The zone list alone is not a combat
implementation.

## Equipment Families

Norns uses three equipment families with eighteen material tiers each. The first
three tiers align with the current prototype; the remaining names are
provisional until the worldbuilding pass is complete.

| Family | Tiers |
| --- | --- |
| Melee | Copper, Tin, Bronze, Iron, Steel, Silver, Blackmetal, Obsidian, Root, Fenris, Carapace, Eitr, Dvergr, Jotunn, Aesir, Bifrost, Yggdrasil, Norn |
| Ranged | Leather, Reinforced, Green Dragon, Blue Dragon, Red Dragon, Black Dragon, Frost Dragon, Tidalscale, Aeonscale |
| Magic | Apprentice, Adept, Mage, Sorcerer, Warlock, Archmage, Frostweave, Abyssweave, Aeonweave |

All equipment is in the Crafted Items category and therefore can resolve to a
quality and participate in the Forge. The tier, equipment slot, combat family,
and quality are separate properties. This produces 12 material tiers x 22
quality tiers per equipment identity; these are data combinations, not separate
Rust enum variants.

## Quality Compatibility

Norns keeps its existing 22-tier quality ladder and Norse display names. The
reference's rarity ladder is useful as a probability comparison, but it does
not replace Norns `Quality` or add quality to non-equipment outputs.

- Raw resources have no quality.
- Refined and smelted materials have no quality.
- Cooked items, potions, ammunition, runes, and glyphs have no quality unless
  a later design explicitly classifies a specific output as Crafted Item.
- Crafted equipment and other explicitly classified Crafted Items resolve to a
  quality-bearing `CraftedItemStack`.
- Forge upgrades preserve crafted identity and consume four identical items.

## Activity Coverage

The implementation checklist is grouped by the existing skill trees.

### Gathering

- Mining: stone, copper, tin, iron, and the remaining zone-gated ores.
- Woodcutting: early timber through late-world and celestial timber.
- Fishing: freshwater, coastal, abyssal, and late-world catches.
- Foraging: plants, herbs, mushrooms, and region-specific ingredients.
- Hunting: hides, bones, meat, and wildlife materials.

### Crafting

- Smithing: ore -> ingot -> weapons and armor.
- Woodworking: logs -> planks -> bows, staves, and wooden tools.
- Leatherworking: hides -> leather -> ranged armor and equipment.
- Cooking: raw fish and meat -> food with defined effects.
- Alchemy: gathered ingredients -> potions with defined effects.
- Runecrafting: essence/materials -> runes and glyphs.

The first missing production chain is:

```text
Iron Ore -> Iron Ingot -> Iron Sword
```

The current prototype temporarily consumes Iron Ore directly in the Iron Sword
recipe. That shortcut must be removed before Smithing is considered complete.

### Combat

- Melee, Ranged, and Seidr equipment progression.
- Zone encounter tables and boss activities.
- Combat XP, loot, gold, defeat behavior, and repeatable activity state.
- Monster and boss definitions for every published zone.

### Additional Skills

- Divinity: offerings, blessings, and divine progression activities.
- Thieving: pickpocketing, stalls, heists, vaults, detection, and rewards.

These skills require new progression and activity models; they should not be
represented as aliases of Gathering or Crafting.

## Implementation Order

1. Complete item taxonomy with refined materials, cooked items, consumables,
   equipment slots, and explicit quality eligibility.
2. Add Iron Ingot and change Smithing to use the ore -> ingot -> sword chain.
3. Generalize production activities around deterministic recipes and
   activity-layer inventory transactions.
4. Add the remaining gathering tables and production recipes by tier.
5. Add equipment families, combat stats, and quality-bearing gear.
6. Add zones, monsters, bosses, loot, and gold sinks.
7. Add Divinity and Thieving as first-class skills and activities.
8. Add lore and endgame/Celestial content after the underlying activity models
   are stable.

## Explicitly Unspecified

The reference does not provide enough information to implement these safely:

- Exact recipe ingredients and quantities.
- Activity durations and offline progression rules.
- Combat formulas, equipment stats, and boss mechanics.
- Drop probabilities beyond the displayed rarity ladder.
- Potion, food, rune, glyph, and blessing effects.
- Thieving detection and heist failure rules.
- Stormcaller and Celestial Realm numeric requirements.

These remain balance/design placeholders rather than silently inferred values.
