use super::{
    CraftedItem, CraftedItemStack, EquipmentError, EquipmentLoadout, EquipmentSlot, ForgeError,
    ForgeOutcome, ItemKind, ItemStack, Quality, forge,
};
use crate::{
    crafting::{SmeltingOutcome, SmithingOutcome},
    economy::Gold,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// One inventory entry, either unresolved or quality-resolved.
pub enum InventoryStack {
    /// Resource or refined-material stack.
    Item(ItemStack),
    /// Quality-resolved crafted stack.
    Crafted(CraftedItemStack),
}

impl InventoryStack {
    #[must_use]
    /// Returns the quantity represented by the entry.
    pub const fn quantity(self) -> u32 {
        match self {
            Self::Item(stack) => stack.quantity(),
            Self::Crafted(stack) => stack.quantity(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Failure reasons for inventory operations.
pub enum InventoryError {
    /// A zero-sized stack was requested.
    ZeroQuantity,
    /// An unresolved crafted item was supplied where a resolved item was required.
    WrongItemCategory,
    /// The requested quantity was not available.
    InsufficientQuantity {
        /// Quantity currently available.
        available: u32,
        /// Quantity requested by the operation.
        required: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Failure reasons for applying a gameplay transaction.
pub enum InventoryTransactionError {
    /// The inventory could not satisfy the transaction.
    Inventory(InventoryError),
    /// The Forge rejected the transaction before mutation.
    Forge(ForgeError),
    /// The item could not be placed in the requested equipment slot.
    Equipment(EquipmentError),
}

impl From<InventoryError> for InventoryTransactionError {
    fn from(error: InventoryError) -> Self {
        Self::Inventory(error)
    }
}

impl From<ForgeError> for InventoryTransactionError {
    fn from(error: ForgeError) -> Self {
        Self::Forge(error)
    }
}

impl From<EquipmentError> for InventoryTransactionError {
    fn from(error: EquipmentError) -> Self {
        Self::Equipment(error)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// Player-owned item stacks and transaction operations.
pub struct Inventory {
    stacks: Vec<InventoryStack>,
}

impl Inventory {
    #[must_use]
    /// Creates an empty inventory.
    pub const fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    #[must_use]
    /// Returns the current inventory entries.
    pub fn stacks(&self) -> &[InventoryStack] {
        &self.stacks
    }

    /// Adds an unqualified resource or refined-material stack.
    ///
    /// # Errors
    ///
    /// Returns `WrongItemCategory` for unresolved crafted items.
    pub fn add_item(&mut self, stack: ItemStack) -> Result<(), InventoryError> {
        if matches!(stack.kind(), ItemKind::CraftedItem(_)) {
            return Err(InventoryError::WrongItemCategory);
        }

        if let Some(existing) = self.stacks.iter_mut().find(|existing| {
            matches!(existing, InventoryStack::Item(existing) if existing.kind() == stack.kind())
        }) {
            let InventoryStack::Item(existing) = existing else {
                unreachable!()
            };
            *existing = ItemStack::new(
                existing.kind(),
                existing.quantity().saturating_add(stack.quantity()),
            )
            .ok_or(InventoryError::ZeroQuantity)?;
        } else {
            self.stacks.push(InventoryStack::Item(stack));
        }

        Ok(())
    }

    /// Adds and merges a quality-resolved crafted-item stack.
    ///
    /// # Errors
    ///
    /// Returns `ZeroQuantity` if a quantity cannot be represented as a stack.
    pub fn add_crafted(&mut self, stack: CraftedItemStack) -> Result<(), InventoryError> {
        if let Some(existing) = self.stacks.iter_mut().find(|existing| {
            matches!(existing, InventoryStack::Crafted(existing) if
                existing.item() == stack.item() && existing.quality() == stack.quality())
        }) {
            let InventoryStack::Crafted(existing) = existing else {
                unreachable!()
            };
            *existing = CraftedItemStack::new(
                existing.item(),
                existing.quality(),
                existing.quantity().saturating_add(stack.quantity()),
            )
            .ok_or(InventoryError::ZeroQuantity)?;
        } else {
            self.stacks.push(InventoryStack::Crafted(stack));
        }

        Ok(())
    }

    /// Removes a quantity of an unqualified item stack.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientQuantity` when the requested stack is unavailable
    /// or the requested quantity is zero.
    pub fn remove_item(
        &mut self,
        kind: ItemKind,
        quantity: u32,
    ) -> Result<ItemStack, InventoryError> {
        let index = self.stacks.iter().position(|existing| {
            matches!(existing, InventoryStack::Item(existing) if existing.kind() == kind)
        });
        let Some(index) = index else {
            return Err(InventoryError::InsufficientQuantity {
                available: 0,
                required: quantity,
            });
        };

        let InventoryStack::Item(existing) = self.stacks[index] else {
            unreachable!()
        };
        if existing.quantity() < quantity || quantity == 0 {
            return Err(InventoryError::InsufficientQuantity {
                available: existing.quantity(),
                required: quantity,
            });
        }

        let removed = ItemStack::new(kind, quantity).ok_or(InventoryError::ZeroQuantity)?;
        let remaining = existing.quantity() - quantity;
        if remaining == 0 {
            self.stacks.remove(index);
        } else {
            self.stacks[index] = InventoryStack::Item(
                ItemStack::new(kind, remaining).ok_or(InventoryError::ZeroQuantity)?,
            );
        }

        Ok(removed)
    }

    /// Removes a quantity of a specific crafted item and quality.
    ///
    /// # Errors
    ///
    /// Returns `InsufficientQuantity` when the requested stack is unavailable
    /// or the requested quantity is zero.
    pub fn remove_crafted(
        &mut self,
        item: CraftedItem,
        quality: Quality,
        quantity: u32,
    ) -> Result<CraftedItemStack, InventoryError> {
        let index = self.stacks.iter().position(|existing| {
            matches!(existing, InventoryStack::Crafted(existing) if
                existing.item() == item && existing.quality() == quality)
        });
        let Some(index) = index else {
            return Err(InventoryError::InsufficientQuantity {
                available: 0,
                required: quantity,
            });
        };

        let InventoryStack::Crafted(existing) = self.stacks[index] else {
            unreachable!()
        };
        if existing.quantity() < quantity || quantity == 0 {
            return Err(InventoryError::InsufficientQuantity {
                available: existing.quantity(),
                required: quantity,
            });
        }

        let removed =
            CraftedItemStack::new(item, quality, quantity).ok_or(InventoryError::ZeroQuantity)?;
        let remaining = existing.quantity() - quantity;
        if remaining == 0 {
            self.stacks.remove(index);
        } else {
            self.stacks[index] = InventoryStack::Crafted(
                CraftedItemStack::new(item, quality, remaining)
                    .ok_or(InventoryError::ZeroQuantity)?,
            );
        }

        Ok(removed)
    }

    /// Applies a completed smelting outcome to inventory.
    ///
    /// # Errors
    ///
    /// Returns `Inventory` when the required input is unavailable.
    pub fn apply_smelting(
        &mut self,
        outcome: SmeltingOutcome,
    ) -> Result<(), InventoryTransactionError> {
        self.remove_item(outcome.consumed().kind(), outcome.consumed().quantity())?;
        self.add_item(outcome.produced())?;
        Ok(())
    }

    /// Applies a completed Smithing outcome to inventory.
    ///
    /// # Errors
    ///
    /// Returns `Inventory` when the required input is unavailable.
    pub fn apply_smithing(
        &mut self,
        outcome: SmithingOutcome,
    ) -> Result<(), InventoryTransactionError> {
        self.remove_item(outcome.consumed().kind(), outcome.consumed().quantity())?;
        self.add_crafted(outcome.produced())?;
        Ok(())
    }

    /// Applies a Forge transaction after validating its inventory and gold.
    ///
    /// # Errors
    ///
    /// Returns `Forge` when the Forge requirements fail, or `Inventory` when
    /// the input stack is unavailable.
    pub fn apply_forge(
        &mut self,
        stack: CraftedItemStack,
        available_gold: Gold,
    ) -> Result<ForgeOutcome, InventoryTransactionError> {
        let outcome = forge(stack, available_gold)?;
        self.remove_crafted(stack.item(), stack.quality(), outcome.consumed_items())?;
        self.add_crafted(outcome.produced())?;
        Ok(outcome)
    }

    /// Equips one item from inventory and returns the replaced item, if any.
    ///
    /// # Errors
    ///
    /// Returns `Inventory` when the item is unavailable, or `Equipment` when
    /// the item cannot occupy its loadout slot. Slot validation occurs before
    /// inventory mutation.
    pub fn equip(
        &mut self,
        item: CraftedItemStack,
        loadout: &mut EquipmentLoadout,
    ) -> Result<Option<CraftedItemStack>, InventoryTransactionError> {
        if item.item().slot() != EquipmentSlot::MainHand {
            return Err(EquipmentError::WrongSlot.into());
        }

        self.remove_crafted(item.item(), item.quality(), 1)?;
        let replaced = loadout.equip(
            CraftedItemStack::new(item.item(), item.quality(), 1)
                .ok_or(InventoryError::ZeroQuantity)?,
        )?;
        if let Some(replaced) = replaced {
            self.add_crafted(replaced)?;
        }
        Ok(replaced)
    }

    /// Unequips an item and returns it to inventory.
    ///
    /// # Errors
    ///
    /// Returns `Inventory` only if the returned item cannot be added. An empty
    /// slot returns `Ok(None)` without changing inventory.
    pub fn unequip(
        &mut self,
        slot: EquipmentSlot,
        loadout: &mut EquipmentLoadout,
    ) -> Result<Option<CraftedItemStack>, InventoryTransactionError> {
        let Some(item) = loadout.unequip(slot) else {
            return Ok(None);
        };
        self.add_crafted(item)?;
        Ok(Some(item))
    }
}

#[cfg(test)]
mod tests {
    use super::{Inventory, InventoryError, InventoryStack};
    use crate::crafting::{IRON_INGOT, IRON_SWORD, smelt, smith};
    use crate::economy::Gold;
    use crate::item::{
        CraftedItem, CraftedItemStack, EquipmentLoadout, EquipmentSlot, ItemKind, MaterialTier,
        Quality, Resource,
    };

    #[test]
    fn resource_stacks_merge_and_split() {
        let mut inventory = Inventory::new();
        let ore = ItemKind::Resource(Resource::IronOre);
        inventory
            .add_item(crate::item::ItemStack::new(ore, 4).unwrap())
            .unwrap();
        inventory
            .add_item(crate::item::ItemStack::new(ore, 3).unwrap())
            .unwrap();

        let removed = inventory.remove_item(ore, 2).unwrap();

        assert_eq!(removed.quantity(), 2);
        assert_eq!(inventory.stacks().len(), 1);
        assert_eq!(inventory.stacks()[0].quantity(), 5);
    }

    #[test]
    fn crafted_stacks_only_merge_when_quality_matches() {
        let mut inventory = Inventory::new();
        let item = CraftedItem::Sword(MaterialTier::Iron);
        inventory
            .add_crafted(CraftedItemStack::new(item, Quality::Rare, 2).unwrap())
            .unwrap();
        inventory
            .add_crafted(CraftedItemStack::new(item, Quality::Epic, 2).unwrap())
            .unwrap();

        assert_eq!(inventory.stacks().len(), 2);
        assert!(matches!(inventory.stacks()[0], InventoryStack::Crafted(_)));
    }

    #[test]
    fn crafted_items_cannot_enter_unresolved_item_stacks() {
        let mut inventory = Inventory::new();
        let stack = crate::item::ItemStack::new(
            ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Iron)),
            1,
        )
        .unwrap();

        assert_eq!(
            inventory.add_item(stack),
            Err(InventoryError::WrongItemCategory)
        );
    }

    #[test]
    fn activity_outcomes_mutate_inventory() {
        let mut inventory = Inventory::new();
        let ore = ItemKind::Resource(Resource::IronOre);
        inventory
            .add_item(crate::item::ItemStack::new(ore, 2).unwrap())
            .unwrap();

        inventory.apply_smelting(smelt(IRON_INGOT)).unwrap();
        assert_eq!(inventory.stacks().len(), 1);
        inventory
            .add_item(
                crate::item::ItemStack::new(
                    ItemKind::RefinedMaterial(crate::item::RefinedMaterial::Ingot(
                        MaterialTier::Iron,
                    )),
                    2,
                )
                .unwrap(),
            )
            .unwrap();
        inventory
            .apply_smithing(smith(
                IRON_SWORD,
                crate::item::QualityRoll::new(0).unwrap(),
                1,
            ))
            .unwrap();

        assert!(
            inventory
                .stacks()
                .iter()
                .any(|stack| matches!(stack, InventoryStack::Crafted(_)))
        );
    }

    #[test]
    fn forge_application_consumes_four_and_adds_one() {
        let mut inventory = Inventory::new();
        let item = CraftedItem::Sword(MaterialTier::Iron);
        inventory
            .add_crafted(CraftedItemStack::new(item, Quality::Rare, 4).unwrap())
            .unwrap();

        let outcome = inventory
            .apply_forge(
                CraftedItemStack::new(item, Quality::Rare, 4).unwrap(),
                Gold::new(1_000_000),
            )
            .unwrap();

        assert_eq!(outcome.consumed_items(), 4);
        assert_eq!(inventory.stacks().len(), 1);
    }

    #[test]
    fn equipping_moves_one_item_and_returns_replaced_item() {
        let item = CraftedItem::Sword(MaterialTier::Iron);
        let first = CraftedItemStack::new(item, Quality::Rare, 2).unwrap();
        let second = CraftedItemStack::new(item, Quality::Epic, 1).unwrap();
        let mut inventory = Inventory::new();
        let mut loadout = EquipmentLoadout::new();
        inventory.add_crafted(first).unwrap();
        inventory.add_crafted(second).unwrap();
        let first_equipped = CraftedItemStack::new(item, Quality::Rare, 1).unwrap();

        assert_eq!(inventory.equip(first, &mut loadout), Ok(None));
        assert_eq!(inventory.stacks()[0].quantity(), 1);
        assert_eq!(
            inventory.equip(second, &mut loadout),
            Ok(Some(first_equipped))
        );
        assert_eq!(loadout.main_hand(), Some(second));
    }

    #[test]
    fn unequipping_returns_the_item_to_inventory() {
        let item = CraftedItem::Sword(MaterialTier::Iron);
        let equipped = CraftedItemStack::new(item, Quality::Rare, 1).unwrap();
        let mut inventory = Inventory::new();
        let mut loadout = EquipmentLoadout::new();
        loadout.equip(equipped).unwrap();

        assert_eq!(
            inventory.unequip(EquipmentSlot::MainHand, &mut loadout),
            Ok(Some(equipped))
        );
        assert_eq!(loadout.main_hand(), None);
        assert_eq!(inventory.stacks(), &[InventoryStack::Crafted(equipped)]);
    }
}
