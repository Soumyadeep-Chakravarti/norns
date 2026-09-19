# Game Design

## Core Loop

**Locked design:**

```text
Gather
   |
   v
Craft
   |
   v
Improve equipment and economy
   |
   v
Access harder activities
   |
   v
Gather better resources
```

Combat intersects with this loop by providing dangerous-region access,
encounters, rare materials, and progression opportunities.

## Gathering

**Locked design:** Gathering skills extract resources from the world.

- Mining
- Woodcutting
- Fishing
- Foraging
- Hunting

Hunting belongs to Gathering when it obtains materials from wildlife and
related activities. Directly fighting enemies belongs to Combat.

## Crafting

**Locked design:** Crafting transforms resources into useful items.

- Smithing
- Woodworking
- Cooking
- Leatherworking
- Alchemy
- Runecrafting

Crafting should be economically meaningful rather than merely a
self-sufficient progression checklist. Specialization should make individual
players desirable producers of particular goods.

## Combat

**Locked design:** Combat families are Melee, Ranged, and Seidr. Norns does not
currently plan to include PvP. Combat supports cooperative encounters,
dangerous regions, bosses, expeditions, and progression.

## Economy

**Locked design:** The economy is player-driven. Planned interactions include
resource sales, specialized crafting, trading, markets, crafting commissions,
gifting, clans, and economic cooperation.

Systems such as the Forge intentionally consume eligible items and currency to
counter long-term inflation in an idle game.

## Multiplayer Philosophy

**Locked design:** Multiplayer should make the world inhabited without
requiring players to fight each other. Players should benefit from becoming
specialists and interacting with other specialists.

## Current Vertical Slice

**Current implementation:** Mining is the first gathering vertical slice, with
Stone, Copper Ore, and Tin Ore. Mining grants broad mining XP and resource
specialization XP alongside an `ItemStack` containing resource identity and a
positive quantity. Raw resources have no quality, and Mining does not roll it.

**Balance placeholder:** Node level requirements, cycle durations, and XP
rewards are provisional.
