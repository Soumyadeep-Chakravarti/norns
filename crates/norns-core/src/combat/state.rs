use super::{enemy::EnemyDefinition, stats::CombatantStats};

/// Health state for both sides of an encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatState {
    pub(crate) player_health: u32,
    pub(crate) enemy_health: u32,
}

impl CombatState {
    /// Creates a full-health encounter state.
    #[must_use]
    pub const fn new(player: CombatantStats, enemy: EnemyDefinition) -> Self {
        Self {
            player_health: player.max_health(),
            enemy_health: enemy.stats().max_health(),
        }
    }

    pub(crate) const fn with_health(player_health: u32, enemy_health: u32) -> Self {
        Self {
            player_health,
            enemy_health,
        }
    }

    /// Returns current player health.
    #[must_use]
    pub const fn player_health(self) -> u32 {
        self.player_health
    }
    /// Returns current enemy health.
    #[must_use]
    pub const fn enemy_health(self) -> u32 {
        self.enemy_health
    }
}
