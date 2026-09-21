use crate::economy::Gold;
use crate::item::{CombatFamily, ItemStack, Resource};
use crate::progression::Experience;

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

/// Player-supplied deterministic input for one attack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackInput {
    critical: bool,
}

impl AttackInput {
    /// Creates an attack input with an explicit critical-hit result.
    #[must_use]
    pub const fn new(critical: bool) -> Self {
        Self { critical }
    }

    /// Returns whether this attack is critical.
    #[must_use]
    pub const fn critical(self) -> bool {
        self.critical
    }
}

/// Health state for both sides of an encounter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatState {
    player_health: u32,
    enemy_health: u32,
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

/// Result of one side dealing damage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackResult {
    damage: u32,
    target_health: u32,
    target_defeated: bool,
}

impl AttackResult {
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

/// Rewards granted when an encounter is won.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CombatReward {
    combat_xp: Experience,
    specialization_xp: Experience,
    loot: Option<ItemStack>,
    gold: Gold,
}

impl CombatReward {
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

const fn resolve_damage(attacker_damage: u32, defender_defense: u32, critical: bool) -> u32 {
    let mitigated = attacker_damage.saturating_sub(defender_defense);
    let base = if mitigated == 0 { 1 } else { mitigated };
    if critical {
        base.saturating_mul(2)
    } else {
        base
    }
}

const fn attack(
    attacker_damage: u32,
    defender_defense: u32,
    target_health: u32,
    critical: bool,
) -> AttackResult {
    let damage = resolve_damage(attacker_damage, defender_defense, critical);
    let target_health = target_health.saturating_sub(damage);
    AttackResult {
        damage,
        target_health,
        target_defeated: target_health == 0,
    }
}

/// Resolves one deterministic combat round without timers or RNG.
///
/// The server supplies the critical-hit result in `input`. If the player wins,
/// the outcome includes the enemy's XP, loot, and gold. If the enemy survives,
/// it attacks once using its deterministic base damage.
///
/// # Errors
///
/// Returns [`CombatError::PlayerDefeated`] or
/// [`CombatError::EnemyDefeated`] when the supplied state is already terminal.
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

    let player_attack = attack(
        player.damage(),
        enemy.stats().defense(),
        state.enemy_health(),
        input.critical(),
    );
    let state_after_player = CombatState {
        player_health: state.player_health(),
        enemy_health: player_attack.target_health(),
    };

    if player_attack.target_defeated() {
        return Ok(RoundOutcome {
            player_attack,
            enemy_attack: None,
            state: state_after_player,
            reward: Some(CombatReward {
                combat_xp: enemy.combat_xp(),
                specialization_xp: enemy.specialization_xp(),
                loot: enemy.loot(),
                gold: enemy.gold(),
            }),
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
        state: CombatState {
            player_health: enemy_attack.target_health(),
            enemy_health: state_after_player.enemy_health(),
        },
        reward: None,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        AttackInput, CombatError, CombatState, CombatantStats, EnemyKind, GOBLIN_CHIEFTAIN,
        resolve_round,
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
        let enemy = super::EnemyDefinition::new(
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
        let defeated_player = CombatState {
            player_health: 0,
            enemy_health: 10,
        };
        assert_eq!(
            resolve_round(
                PLAYER,
                GOBLIN_CHIEFTAIN,
                defeated_player,
                AttackInput::new(false)
            ),
            Err(CombatError::PlayerDefeated)
        );

        let defeated_enemy = CombatState {
            player_health: 10,
            enemy_health: 0,
        };
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
