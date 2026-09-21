use super::{input::AttackInput, state::CombatState, stats::CombatantStats};

/// Result of one side dealing damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackResult {
    damage: u32,
    target_health: u32,
    target_defeated: bool,
}

impl AttackResult {
    pub(crate) const fn new(damage: u32, target_health: u32) -> Self {
        Self {
            damage,
            target_health,
            target_defeated: target_health == 0,
        }
    }
    /// Returns damage dealt.
    #[must_use]
    pub const fn damage(self) -> u32 {
        self.damage
    }
    /// Returns target health after the attack.
    #[must_use]
    pub const fn target_health(self) -> u32 {
        self.target_health
    }
    /// Returns whether the attack defeated its target.
    #[must_use]
    pub const fn target_defeated(self) -> bool {
        self.target_defeated
    }
}

pub(crate) const fn attack(
    attacker_damage: u32,
    defender_defense: u32,
    target_health: u32,
    critical: bool,
) -> AttackResult {
    let mitigated = attacker_damage.saturating_sub(defender_defense);
    let base = if mitigated == 0 { 1 } else { mitigated };
    let damage = if critical {
        base.saturating_mul(2)
    } else {
        base
    };
    AttackResult::new(damage, target_health.saturating_sub(damage))
}

pub(crate) const fn player_attack(
    player: CombatantStats,
    enemy: CombatantStats,
    state: CombatState,
    input: AttackInput,
) -> AttackResult {
    attack(
        player.damage(),
        enemy.defense(),
        state.enemy_health(),
        input.critical(),
    )
}
