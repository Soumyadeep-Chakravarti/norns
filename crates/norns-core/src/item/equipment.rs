use super::{CraftedItem, CraftedItemStack, MaterialTier};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Equipment slot occupied by an item.
pub enum EquipmentSlot {
    /// Main-hand weapon slot.
    MainHand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Combat family that uses an item.
pub enum CombatFamily {
    /// Physical melee combat.
    Melee,
    /// Physical ranged combat.
    Ranged,
    /// Seidr combat.
    Seidr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Provisional base stats before quality and player modifiers.
pub struct EquipmentStats {
    damage: u32,
    defense: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// A player's currently equipped quality-resolved items.
pub struct EquipmentLoadout {
    main_hand: Option<CraftedItemStack>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Failure reasons for loadout operations.
pub enum EquipmentError {
    /// The item does not belong in the requested slot.
    WrongSlot,
}

impl EquipmentLoadout {
    /// Creates an empty loadout.
    #[must_use]
    pub const fn new() -> Self {
        Self { main_hand: None }
    }

    /// Returns the item equipped in the main-hand slot.
    #[must_use]
    pub const fn main_hand(self) -> Option<CraftedItemStack> {
        self.main_hand
    }

    /// Equips an item and returns the item previously occupying its slot.
    ///
    /// # Errors
    ///
    /// Returns [`EquipmentError::WrongSlot`] if the item's metadata does not
    /// map to the requested equipment slot.
    pub fn equip(
        &mut self,
        item: CraftedItemStack,
    ) -> Result<Option<CraftedItemStack>, EquipmentError> {
        if item.item().slot() != EquipmentSlot::MainHand {
            return Err(EquipmentError::WrongSlot);
        }

        Ok(self.main_hand.replace(item))
    }

    /// Removes and returns the item in a slot.
    pub const fn unequip(&mut self, slot: EquipmentSlot) -> Option<CraftedItemStack> {
        match slot {
            EquipmentSlot::MainHand => self.main_hand.take(),
        }
    }
}

impl EquipmentStats {
    #[must_use]
    /// Returns base damage.
    pub const fn damage(self) -> u32 {
        self.damage
    }

    #[must_use]
    /// Returns base defense.
    pub const fn defense(self) -> u32 {
        self.defense
    }
}

impl CraftedItem {
    #[must_use]
    /// Returns the material tier used by the item.
    pub const fn material_tier(self) -> MaterialTier {
        match self {
            Self::Axe(tier) | Self::Sword(tier) => tier,
        }
    }

    #[must_use]
    /// Returns the equipment slot.
    pub const fn slot(self) -> EquipmentSlot {
        EquipmentSlot::MainHand
    }

    #[must_use]
    /// Returns the combat family.
    pub const fn combat_family(self) -> CombatFamily {
        match self {
            Self::Axe(_) | Self::Sword(_) => CombatFamily::Melee,
        }
    }

    /// Provisional base stats before quality and character modifiers.
    #[must_use]
    pub const fn base_stats(self) -> EquipmentStats {
        let tier = self.material_tier().tier() as u32;
        match self {
            Self::Sword(_) => EquipmentStats {
                damage: tier.saturating_mul(10),
                defense: tier.saturating_mul(2),
            },
            Self::Axe(_) => EquipmentStats {
                damage: tier.saturating_mul(8),
                defense: tier,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CombatFamily, EquipmentLoadout, EquipmentSlot};
    use crate::item::{CraftedItem, CraftedItemStack, MaterialTier, Quality};

    #[test]
    fn equipment_metadata_identifies_melee_main_hand_items() {
        let item = CraftedItem::Sword(MaterialTier::Iron);

        assert_eq!(item.material_tier(), MaterialTier::Iron);
        assert_eq!(item.slot(), EquipmentSlot::MainHand);
        assert_eq!(item.combat_family(), CombatFamily::Melee);
    }

    #[test]
    fn tier_scales_provisional_base_stats() {
        let copper = CraftedItem::Sword(MaterialTier::Copper).base_stats();
        let iron = CraftedItem::Sword(MaterialTier::Iron).base_stats();

        assert_eq!(copper.damage(), 10);
        assert_eq!(copper.defense(), 2);
        assert!(iron.damage() > copper.damage());
        assert!(iron.defense() > copper.defense());
    }

    #[test]
    fn loadout_replaces_and_unequips_main_hand_items() {
        let first = CraftedItemStack::new(CraftedItem::Sword(MaterialTier::Iron), Quality::Rare, 1)
            .unwrap();
        let second =
            CraftedItemStack::new(CraftedItem::Axe(MaterialTier::Iron), Quality::Epic, 1).unwrap();
        let mut loadout = EquipmentLoadout::new();

        assert_eq!(loadout.equip(first), Ok(None));
        assert_eq!(loadout.equip(second), Ok(Some(first)));
        assert_eq!(loadout.main_hand(), Some(second));
        assert_eq!(loadout.unequip(EquipmentSlot::MainHand), Some(second));
        assert_eq!(loadout.main_hand(), None);
    }
}
