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

/// Timing configuration for a repeatable combat activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatActivityDefinition {
    player_attack_interval_seconds: u32,
    enemy_attack_interval_seconds: u32,
}

impl CombatActivityDefinition {
    /// Creates an activity timing definition.
    ///
    /// Intervals are measured by the activity layer, not by core. Both values
    /// must be non-zero when the activity is resolved.
    #[must_use]
    pub const fn new(
        player_attack_interval_seconds: u32,
        enemy_attack_interval_seconds: u32,
    ) -> Self {
        Self {
            player_attack_interval_seconds,
            enemy_attack_interval_seconds,
        }
    }

    /// Returns the player's attack interval in seconds.
    #[must_use]
    pub const fn player_attack_interval_seconds(self) -> u32 {
        self.player_attack_interval_seconds
    }

    /// Returns the enemy's attack interval in seconds.
    #[must_use]
    pub const fn enemy_attack_interval_seconds(self) -> u32 {
        self.enemy_attack_interval_seconds
    }
}

/// Why a combat activity stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatActivityStop {
    /// The enemy was defeated and rewards are available.
    EnemyDefeated,
    /// The player was defeated.
    PlayerDefeated,
    /// The requested elapsed time ended while both combatants survived.
    TimeElapsed,
}

/// Result of resolving an elapsed-time combat activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatActivityOutcome {
    state: CombatState,
    elapsed_seconds: u32,
    player_attacks: u32,
    enemy_attacks: u32,
    stop: CombatActivityStop,
    reward: Option<CombatReward>,
}

impl CombatActivityOutcome {
    /// Returns the final health state.
    #[must_use]
    pub const fn state(self) -> CombatState {
        self.state
    }

    /// Returns elapsed time processed before stopping.
    #[must_use]
    pub const fn elapsed_seconds(self) -> u32 {
        self.elapsed_seconds
    }

    /// Returns the number of player attacks processed.
    #[must_use]
    pub const fn player_attacks(self) -> u32 {
        self.player_attacks
    }

    /// Returns the number of enemy attacks processed.
    #[must_use]
    pub const fn enemy_attacks(self) -> u32 {
        self.enemy_attacks
    }

    /// Returns why processing stopped.
    #[must_use]
    pub const fn stop(self) -> CombatActivityStop {
        self.stop
    }

    /// Returns rewards when the enemy was defeated.
    #[must_use]
    pub const fn reward(self) -> Option<CombatReward> {
        self.reward
    }
}

/// Errors specific to elapsed-time activity processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatActivityError {
    /// An attack interval was zero.
    ZeroAttackInterval,
    /// The activity needed a critical-hit input that was not supplied.
    MissingAttackInput {
        /// Zero-based index of the missing player attack input.
        index: u32,
    },
    /// The initial state was already terminal.
    TerminalState(CombatError),
}

/// Resolves repeated combat events over supplied elapsed time.
///
/// The activity does not sleep, read a clock, generate randomness, or mutate
/// player state. The server supplies elapsed time and one `AttackInput` for
/// each player attack that may occur. Player attacks are processed before enemy
/// attacks when both occur at the same second.
///
/// # Errors
///
/// Returns an error for zero intervals, missing player attack inputs, or a
/// terminal initial state.
pub fn resolve_activity(
    definition: CombatActivityDefinition,
    player: CombatantStats,
    enemy: EnemyDefinition,
    mut state: CombatState,
    elapsed_seconds: u32,
    inputs: &[AttackInput],
) -> Result<CombatActivityOutcome, CombatActivityError> {
    if definition.player_attack_interval_seconds() == 0
        || definition.enemy_attack_interval_seconds() == 0
    {
        return Err(CombatActivityError::ZeroAttackInterval);
    }
    if state.player_health() == 0 {
        return Err(CombatActivityError::TerminalState(
            CombatError::PlayerDefeated,
        ));
    }
    if state.enemy_health() == 0 {
        return Err(CombatActivityError::TerminalState(
            CombatError::EnemyDefeated,
        ));
    }

    let mut player_attacks = 0;
    let mut enemy_attacks = 0;
    for second in 1..=elapsed_seconds {
        if second % definition.player_attack_interval_seconds() == 0 {
            let Some(input) = inputs.get(player_attacks as usize).copied() else {
                return Err(CombatActivityError::MissingAttackInput {
                    index: player_attacks,
                });
            };
            let result = player_attack(player, enemy.stats(), state, input);
            player_attacks = player_attacks.saturating_add(1);
            state = CombatState::with_health(state.player_health(), result.target_health());
            if result.target_defeated() {
                return Ok(CombatActivityOutcome {
                    state,
                    elapsed_seconds: second,
                    player_attacks,
                    enemy_attacks,
                    stop: CombatActivityStop::EnemyDefeated,
                    reward: Some(CombatReward::from_enemy(enemy)),
                });
            }
        }

        if second % definition.enemy_attack_interval_seconds() == 0 {
            let result = attack(
                enemy.stats().damage(),
                player.defense(),
                state.player_health(),
                false,
            );
            enemy_attacks = enemy_attacks.saturating_add(1);
            state = CombatState::with_health(result.target_health(), state.enemy_health());
            if result.target_defeated() {
                return Ok(CombatActivityOutcome {
                    state,
                    elapsed_seconds: second,
                    player_attacks,
                    enemy_attacks,
                    stop: CombatActivityStop::PlayerDefeated,
                    reward: None,
                });
            }
        }
    }

    Ok(CombatActivityOutcome {
        state,
        elapsed_seconds,
        player_attacks,
        enemy_attacks,
        stop: CombatActivityStop::TimeElapsed,
        reward: None,
    })
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

#[cfg(test)]
mod tests {
    use super::{
        CombatActivityDefinition, CombatActivityError, CombatActivityStop, resolve_activity,
    };
    use crate::combat::{AttackInput, CombatState, CombatantStats, GOBLIN_CHIEFTAIN};
    use crate::item::CombatFamily;

    const PLAYER: CombatantStats = CombatantStats::new(CombatFamily::Melee, 100, 20, 5);

    #[test]
    fn activity_stops_with_rewards_when_enemy_dies() {
        let outcome = resolve_activity(
            CombatActivityDefinition::new(2, 3),
            PLAYER,
            GOBLIN_CHIEFTAIN,
            CombatState::new(PLAYER, GOBLIN_CHIEFTAIN),
            10,
            &[AttackInput::new(false), AttackInput::new(true)],
        )
        .expect("activity should resolve");

        assert_eq!(outcome.stop(), CombatActivityStop::EnemyDefeated);
        assert_eq!(outcome.elapsed_seconds(), 4);
        assert_eq!(outcome.player_attacks(), 2);
        assert_eq!(outcome.enemy_attacks(), 1);
        assert!(outcome.reward().is_some());
    }

    #[test]
    fn activity_reports_time_exhaustion_without_rewards() {
        let outcome = resolve_activity(
            CombatActivityDefinition::new(10, 10),
            PLAYER,
            GOBLIN_CHIEFTAIN,
            CombatState::new(PLAYER, GOBLIN_CHIEFTAIN),
            3,
            &[],
        )
        .expect("activity should resolve");

        assert_eq!(outcome.stop(), CombatActivityStop::TimeElapsed);
        assert_eq!(outcome.elapsed_seconds(), 3);
        assert_eq!(outcome.player_attacks(), 0);
        assert_eq!(outcome.enemy_attacks(), 0);
        assert_eq!(outcome.reward(), None);
    }

    #[test]
    fn activity_requires_inputs_for_player_attacks() {
        assert_eq!(
            resolve_activity(
                CombatActivityDefinition::new(1, 2),
                PLAYER,
                GOBLIN_CHIEFTAIN,
                CombatState::new(PLAYER, GOBLIN_CHIEFTAIN),
                1,
                &[],
            ),
            Err(CombatActivityError::MissingAttackInput { index: 0 })
        );
    }
}
