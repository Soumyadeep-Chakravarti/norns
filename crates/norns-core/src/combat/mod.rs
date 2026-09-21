//! Deterministic combat definitions and round resolution.

mod activity;
mod enemy;
mod input;
mod resolution;
mod reward;
mod state;
mod stats;

pub use activity::{
    CombatActivityDefinition, CombatActivityError, CombatActivityOutcome, CombatActivityStop,
    CombatError, RoundOutcome, resolve_activity, resolve_round,
};
pub use enemy::{EnemyDefinition, EnemyKind, GOBLIN_CHIEFTAIN};
pub use input::AttackInput;
pub use resolution::AttackResult;
pub use reward::CombatReward;
pub use state::CombatState;
pub use stats::CombatantStats;

#[cfg(test)]
mod tests {
    use super::{
        AttackInput, CombatError, CombatState, CombatantStats, EnemyDefinition, EnemyKind,
        GOBLIN_CHIEFTAIN, resolve_round,
    };
    use crate::item::CombatFamily;

    const PLAYER: CombatantStats = CombatantStats::new(CombatFamily::Melee, 100, 20, 5);

    #[test]
    fn surviving_round_resolves_player_then_enemy_attack() {
        let state = CombatState::new(PLAYER, GOBLIN_CHIEFTAIN);
        let outcome = resolve_round(PLAYER, GOBLIN_CHIEFTAIN, state, AttackInput::new(false))
            .expect("round should resolve");

        assert_eq!(outcome.player_attack().damage(), 18);
        assert_eq!(outcome.enemy_attack().unwrap().damage(), 3);
        assert_eq!(outcome.state().enemy_health(), 32);
        assert_eq!(outcome.state().player_health(), 97);
        assert_eq!(outcome.reward(), None);
    }

    #[test]
    fn critical_attack_can_end_an_encounter_without_enemy_attack() {
        let enemy = EnemyDefinition::new(
            EnemyKind::Goblin,
            1,
            CombatantStats::new(CombatFamily::Melee, 10, 1, 0),
            5,
            3,
            None,
            crate::economy::Gold::new(7),
        );
        let state = CombatState::new(PLAYER, enemy);
        let outcome = resolve_round(PLAYER, enemy, state, AttackInput::new(true))
            .expect("critical round should resolve");

        assert!(outcome.player_attack().target_defeated());
        assert_eq!(outcome.enemy_attack(), None);
        assert_eq!(outcome.reward().unwrap().combat_xp(), 5);
        assert_eq!(outcome.reward().unwrap().gold().amount(), 7);
    }

    #[test]
    fn terminal_states_are_rejected() {
        let defeated_player = CombatState::with_health(0, 10);
        assert_eq!(
            resolve_round(
                PLAYER,
                GOBLIN_CHIEFTAIN,
                defeated_player,
                AttackInput::new(false)
            ),
            Err(CombatError::PlayerDefeated)
        );

        let defeated_enemy = CombatState::with_health(10, 0);
        assert_eq!(
            resolve_round(
                PLAYER,
                GOBLIN_CHIEFTAIN,
                defeated_enemy,
                AttackInput::new(false)
            ),
            Err(CombatError::EnemyDefeated)
        );
    }
}
