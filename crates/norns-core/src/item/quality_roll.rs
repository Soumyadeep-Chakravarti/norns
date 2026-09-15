use crate::progression::SkillLevel;

use super::Quality;

pub const QUALITY_ROLL_RANGE: u64 = 2_000_000;
pub const BASE_LUCK_BASIS_POINTS: u64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QualityRoll(u64);

impl QualityRoll {
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        if value < QUALITY_ROLL_RANGE {
            Some(Self(value))
        } else {
            None
        }
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Returns the quality-luck multiplier in basis points.
///
/// `10_000` represents 1.00× luck.
///
/// The bonus grows indefinitely with specialization, but at a diminishing
/// rate. Every doubling milestone contributes progressively less additional
/// luck.
#[must_use]
pub fn specialization_luck(level: SkillLevel) -> u64 {
    let mastery = u64::from(level.saturating_sub(1));

    if mastery == 0 {
        return BASE_LUCK_BASIS_POINTS;
    }

    let magnitude = 64 - mastery.leading_zeros();
    let bonus = u64::from(magnitude) * 750;

    BASE_LUCK_BASIS_POINTS.saturating_add(bonus)
}

#[must_use]
pub fn roll_quality(roll: QualityRoll, specialization_level: SkillLevel) -> Quality {
    let luck = specialization_luck(specialization_level);

    let effective_roll = roll.value().saturating_mul(luck) / BASE_LUCK_BASIS_POINTS;

    quality_from_score(effective_roll)
}

const fn quality_from_score(score: u64) -> Quality {
    if score >= threshold(Quality::Primordial) {
        Quality::Primordial
    } else if score >= threshold(Quality::NornTouched) {
        Quality::NornTouched
    } else if score >= threshold(Quality::Worldforged) {
        Quality::Worldforged
    } else if score >= threshold(Quality::AesirTouched) {
        Quality::AesirTouched
    } else if score >= threshold(Quality::Einherjar) {
        Quality::Einherjar
    } else if score >= threshold(Quality::Voidborn) {
        Quality::Voidborn
    } else if score >= threshold(Quality::Jotunnforged) {
        Quality::Jotunnforged
    } else if score >= threshold(Quality::HelForged) {
        Quality::HelForged
    } else if score >= threshold(Quality::Fatebound) {
        Quality::Fatebound
    } else if score >= threshold(Quality::Forgotten) {
        Quality::Forgotten
    } else if score >= threshold(Quality::Eldritch) {
        Quality::Eldritch
    } else if score >= threshold(Quality::Cursed) {
        Quality::Cursed
    } else if score >= threshold(Quality::Runebound) {
        Quality::Runebound
    } else if score >= threshold(Quality::Relic) {
        Quality::Relic
    } else if score >= threshold(Quality::Ancient) {
        Quality::Ancient
    } else if score >= threshold(Quality::Mythic) {
        Quality::Mythic
    } else if score >= threshold(Quality::Legendary) {
        Quality::Legendary
    } else if score >= threshold(Quality::Epic) {
        Quality::Epic
    } else if score >= threshold(Quality::Rare) {
        Quality::Rare
    } else if score >= threshold(Quality::Uncommon) {
        Quality::Uncommon
    } else if score >= threshold(Quality::Common) {
        Quality::Common
    } else {
        Quality::Standard
    }
}

const fn threshold(quality: Quality) -> u64 {
    QUALITY_ROLL_RANGE - QUALITY_ROLL_RANGE / quality.base_rarity()
}

#[cfg(test)]
mod tests {
    use super::{
        BASE_LUCK_BASIS_POINTS, QUALITY_ROLL_RANGE, QualityRoll, roll_quality, specialization_luck,
    };
    use crate::item::Quality;

    #[test]
    fn roll_must_be_inside_range() {
        assert!(QualityRoll::new(0).is_some());
        assert!(QualityRoll::new(QUALITY_ROLL_RANGE - 1).is_some());
        assert!(QualityRoll::new(QUALITY_ROLL_RANGE).is_none());
    }

    #[test]
    fn level_one_has_base_luck() {
        assert_eq!(specialization_luck(1), BASE_LUCK_BASIS_POINTS);
    }

    #[test]
    fn specialization_increases_luck() {
        assert!(specialization_luck(100) > specialization_luck(1));
        assert!(specialization_luck(1_000) > specialization_luck(100));
    }

    #[test]
    fn zero_roll_is_standard() {
        let roll = QualityRoll::new(0).expect("roll should be valid");

        assert_eq!(roll_quality(roll, 1), Quality::Standard);
    }

    #[test]
    fn base_luck_can_roll_common() {
        let roll = QualityRoll::new(1_000_000).expect("roll should be valid");

        assert_eq!(roll_quality(roll, 1), Quality::Common);
    }

    #[test]
    fn maximum_roll_is_primordial_at_base_luck() {
        let roll = QualityRoll::new(QUALITY_ROLL_RANGE - 1).expect("roll should be valid");

        assert_eq!(roll_quality(roll, 1), Quality::Primordial);
    }

    #[test]
    fn specialization_can_improve_same_roll() {
        let roll = QualityRoll::new(900_000).expect("roll should be valid");

        let inexperienced = roll_quality(roll, 1);
        let experienced = roll_quality(roll, 1_000);

        assert!(experienced > inexperienced);
    }
}
