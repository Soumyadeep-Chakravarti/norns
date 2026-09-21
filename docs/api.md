# Core API Guide

`norns-core` contains deterministic game rules. It does not open files, access
the network, generate randomness, or persist player state.

## Data Flow

Activities are calculated first and applied second:

```text
server state + server RNG
          |
          v
core activity function
          |
          v
deterministic outcome
          |
          v
inventory transaction + persistence
```

For example, Smithing resolves a quality-bearing output but does not remove
ingots. The server validates authorization and then calls
`Inventory::apply_smithing` to apply the outcome.

## Item Categories

`ItemKind` deliberately distinguishes three categories:

| Category | Representation | Quality |
| --- | --- | --- |
| Raw resource | `ItemKind::Resource` | Never |
| Refined material | `ItemKind::RefinedMaterial` | Never |
| Crafted item | `ItemKind::CraftedItem` | Always eligible |

`ItemStack` is an unresolved identity and quantity. It is appropriate for raw
resources, refined materials, recipe inputs, and unresolved recipe outputs.
`CraftedItemStack` is the owned representation of a crafted item after quality
has been resolved. `Standard` is an actual quality, not a missing-value marker.

`EquipmentLoadout` stores equipped `CraftedItemStack` values separately from
the inventory. Equipping replaces the item in a slot and returns the previous
item so the caller can return it to inventory. Unequipping returns the item to
the caller; the loadout does not destroy items.

## Quality and Randomness

The server creates a `QualityRoll` in the range `0..2_000_000`. Core then
combines it with specialization level using `roll_quality`. This keeps quality
resolution deterministic while allowing the server to choose its RNG and
randomness policy.

Raw resources and refined materials never receive quality. A mob drop or other
source does not change that category rule. Crafted equipment retains its
material tier and identity while its quality is stored in `CraftedItemStack`.

## Activity Outcomes

`mine`, `smelt`, and `smith` return descriptions of costs and rewards. They do
not mutate an inventory. This makes them suitable for previews, simulations,
replays, and server-side validation.

The corresponding inventory methods are:

- `Inventory::apply_smelting`
- `Inventory::apply_smithing`
- `Inventory::apply_forge`

These methods consume inputs and add outputs. Forge validates its quality,
quantity, and gold requirements before inventory mutation. Failed operations
return an error and do not intentionally consume input stacks.

`Inventory::equip` and `Inventory::unequip` coordinate the ownership boundary
between inventory and `EquipmentLoadout`. Equipping removes one item, places it
in the loadout, and returns the replaced item to inventory. Unequipping moves
the equipped item back to inventory.

## Balance Status

Tier requirements, durations, XP, recipe quantities, equipment stats, Forge
fees, and quality rarity values are currently provisional. They are encoded in
the core because activities need deterministic behavior, but they should be
treated as balance data and tested with simulations before release.

## Extension Rules

When adding a new item:

1. Classify it as raw, refined, cooked, or crafted before defining its stack.
2. Add quality only if it is a `CraftedItem` by design.
3. Keep RNG outside core and pass rolls into deterministic functions.
4. Return an outcome before mutating player state.
5. Add tests for identity, quantity, category quality eligibility, and failure
   behavior.
