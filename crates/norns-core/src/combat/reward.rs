use crate::{economy::Gold, item::ItemStack, progression::Experience};

use super::enemy::EnemyDefinition;

/// Rewards granted when an encounter is won.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatReward {
    combat_xp: Experience,
    specialization_xp: Experience,
    loot: Option<ItemStack>,
    gold: Gold,
}

impl CombatReward {
    pub(crate) const fn from_enemy(enemy: EnemyDefinition) -> Self {
        Self {
            combat_xp: enemy.combat_xp(),
            specialization_xp: enemy.specialization_xp(),
            loot: enemy.loot(),
            gold: enemy.gold(),
        }
    }
    /// Returns broad combat XP.
    #[must_use]
    pub const fn combat_xp(self) -> Experience {
        self.combat_xp
    }
    /// Returns specialization XP.
    #[must_use]
    pub const fn specialization_xp(self) -> Experience {
        self.specialization_xp
    }
    /// Returns the optional item drop.
    #[must_use]
    pub const fn loot(self) -> Option<ItemStack> {
        self.loot
    }
    /// Returns gold awarded.
    #[must_use]
    pub const fn gold(self) -> Gold {
        self.gold
    }
}
