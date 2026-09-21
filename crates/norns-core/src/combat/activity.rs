use super::{
    enemy::EnemyDefinition,
    input::AttackInput,
    resolution::{AttackResult, attack, player_attack},
    reward::CombatReward,
    state::CombatState,
    stats::CombatantStats,
};

/// Result of resolving one player/enemy combat round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundOutcome {
    player_attack: AttackResult,
    enemy_attack: Option<AttackResult>,
    state: CombatState,
    reward: Option<CombatReward>,
}

impl RoundOutcome {
    /// Returns the player's attack result.
    #[must_use]
    pub const fn player_attack(self) -> AttackResult {
        self.player_attack
    }
    /// Returns the enemy attack result, or `None` if the enemy was defeated.
    #[must_use]
    pub const fn enemy_attack(self) -> Option<AttackResult> {
        self.enemy_attack
    }
    /// Returns the updated encounter state.
    #[must_use]
    pub const fn state(self) -> CombatState {
        self.state
    }
    /// Returns rewards when the player defeated the enemy.
    #[must_use]
    pub const fn reward(self) -> Option<CombatReward> {
        self.reward
    }
}

/// Errors that prevent a combat round from resolving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatError {
    /// The player has no health remaining.
    PlayerDefeated,
    /// The enemy has already been defeated.
    EnemyDefeated,
}

/// Resolves one deterministic combat round without timers or RNG.
///
/// The server supplies the critical-hit result in `input`. If the player wins,
/// the outcome includes the enemy's XP, loot, and gold. If the enemy survives,
/// it attacks once using its deterministic base damage.
///
/// # Errors
///
/// Returns [`CombatError::PlayerDefeated`] or [`CombatError::EnemyDefeated`] when
/// the supplied state is already terminal.
pub const fn resolve_round(
    player: CombatantStats,
    enemy: EnemyDefinition,
    state: CombatState,
    input: AttackInput,
) -> Result<RoundOutcome, CombatError> {
    if state.player_health() == 0 {
        return Err(CombatError::PlayerDefeated);
    }
    if state.enemy_health() == 0 {
        return Err(CombatError::EnemyDefeated);
    }

    let player_attack = player_attack(player, enemy.stats(), state, input);
    let state_after_player =
        CombatState::with_health(state.player_health(), player_attack.target_health());
    if player_attack.target_defeated() {
        return Ok(RoundOutcome {
            player_attack,
            enemy_attack: None,
            state: state_after_player,
            reward: Some(CombatReward::from_enemy(enemy)),
        });
    }

    let enemy_attack = attack(
        enemy.stats().damage(),
        player.defense(),
        state.player_health(),
        false,
    );
    Ok(RoundOutcome {
        player_attack,
        enemy_attack: Some(enemy_attack),
        state: CombatState::with_health(
            enemy_attack.target_health(),
            state_after_player.enemy_health(),
        ),
        reward: None,
    })
}
