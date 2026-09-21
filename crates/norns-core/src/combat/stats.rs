use crate::item::CombatFamily;

/// Basic combat attributes used by deterministic damage resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatantStats {
    family: CombatFamily,
    max_health: u32,
    damage: u32,
    defense: u32,
}

impl CombatantStats {
    /// Creates a combat stat block.
    #[must_use]
    pub const fn new(family: CombatFamily, max_health: u32, damage: u32, defense: u32) -> Self {
        Self {
            family,
            max_health,
            damage,
            defense,
        }
    }

    /// Returns the combat family.
    #[must_use]
    pub const fn family(self) -> CombatFamily {
        self.family
    }

    /// Returns maximum health.
    #[must_use]
    pub const fn max_health(self) -> u32 {
        self.max_health
    }

    /// Returns outgoing base damage.
    #[must_use]
    pub const fn damage(self) -> u32 {
        self.damage
    }

    /// Returns incoming damage mitigation.
    #[must_use]
    pub const fn defense(self) -> u32 {
        self.defense
    }
}
