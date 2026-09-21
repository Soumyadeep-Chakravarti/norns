use crate::economy::Gold;
use crate::item::{CombatFamily, ItemStack, Resource};
use crate::progression::Experience;

use super::stats::CombatantStats;

/// Content identity for an enemy definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyKind {
    /// A basic Verdant Meadows enemy.
    Goblin,
    /// The Verdant Meadows boss.
    GoblinChieftain,
}

/// Static data used to populate a combat encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnemyDefinition {
    kind: EnemyKind,
    level: u32,
    stats: CombatantStats,
    combat_xp: Experience,
    specialization_xp: Experience,
    loot: Option<ItemStack>,
    gold: Gold,
}

impl EnemyDefinition {
    /// Creates an enemy definition for content tables.
    #[must_use]
    pub const fn new(
        kind: EnemyKind,
        level: u32,
        stats: CombatantStats,
        combat_xp: Experience,
        specialization_xp: Experience,
        loot: Option<ItemStack>,
        gold: Gold,
    ) -> Self {
        Self {
            kind,
            level,
            stats,
            combat_xp,
            specialization_xp,
            loot,
            gold,
        }
    }

    /// Returns the enemy identity.
    #[must_use]
    pub const fn kind(self) -> EnemyKind {
        self.kind
    }
    /// Returns the enemy level.
    #[must_use]
    pub const fn level(self) -> u32 {
        self.level
    }
    /// Returns the enemy stat block.
    #[must_use]
    pub const fn stats(self) -> CombatantStats {
        self.stats
    }
    /// Returns broad combat XP awarded on victory.
    #[must_use]
    pub const fn combat_xp(self) -> Experience {
        self.combat_xp
    }
    /// Returns combat specialization XP awarded on victory.
    #[must_use]
    pub const fn specialization_xp(self) -> Experience {
        self.specialization_xp
    }
    /// Returns the optional item drop.
    #[must_use]
    pub const fn loot(self) -> Option<ItemStack> {
        self.loot
    }
    /// Returns gold awarded on victory.
    #[must_use]
    pub const fn gold(self) -> Gold {
        self.gold
    }
}

/// Provisional Goblin Chieftain content entry for Verdant Meadows.
pub const GOBLIN_CHIEFTAIN: EnemyDefinition = {
    let Some(loot) = ItemStack::new(crate::item::ItemKind::Resource(Resource::Stone), 1) else {
        unreachable!()
    };
    EnemyDefinition::new(
        EnemyKind::GoblinChieftain,
        1,
        CombatantStats::new(CombatFamily::Melee, 50, 8, 2),
        25,
        25,
        Some(loot),
        Gold::new(100),
    )
};
