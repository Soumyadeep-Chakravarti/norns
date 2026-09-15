use crate::economy::Gold;

use super::Quality;

pub const FORGE_ITEM_COST: u32 = 4;

const BASE_FORGE_COST: u64 = 100;
const FORGE_COST_MULTIPLIER: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForgeOutcome {
    consumed_items: u32,
    produced_quality: Quality,
    gold_spent: Gold,
    gold_remaining: Gold,
}

impl ForgeOutcome {
    #[must_use]
    pub const fn consumed_items(self) -> u32 {
        self.consumed_items
    }

    #[must_use]
    pub const fn produced_quality(self) -> Quality {
        self.produced_quality
    }

    #[must_use]
    pub const fn gold_spent(self) -> Gold {
        self.gold_spent
    }

    #[must_use]
    pub const fn gold_remaining(self) -> Gold {
        self.gold_remaining
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForgeError {
    FinalQuality,
    InsufficientItems { required: u32, available: u32 },
    InsufficientGold { required: Gold, available: Gold },
}

#[must_use]
pub const fn forge_cost(quality: Quality) -> Option<Gold> {
    let Some(next_quality) = quality.next() else {
        return None;
    };

    let mut cost = BASE_FORGE_COST;
    let mut tier = 1;

    while tier < next_quality.tier() {
        cost = cost.saturating_mul(FORGE_COST_MULTIPLIER);
        tier += 1;
    }

    Some(Gold::new(cost))
}

/// Forges four items of the same quality into one item of the next quality.
///
/// # Errors
///
/// Returns [`ForgeError::FinalQuality`] if the input quality is already
/// [`Quality::Primordial`].
///
/// Returns [`ForgeError::InsufficientItems`] if fewer than
/// [`FORGE_ITEM_COST`] items are available.
///
/// Returns [`ForgeError::InsufficientGold`] if the available gold is less
/// than the required forge cost.
pub const fn forge(
    quality: Quality,
    available_items: u32,
    available_gold: Gold,
) -> Result<ForgeOutcome, ForgeError> {
    let Some(produced_quality) = quality.next() else {
        return Err(ForgeError::FinalQuality);
    };

    if available_items < FORGE_ITEM_COST {
        return Err(ForgeError::InsufficientItems {
            required: FORGE_ITEM_COST,
            available: available_items,
        });
    }

    let Some(cost) = forge_cost(quality) else {
        return Err(ForgeError::FinalQuality);
    };

    let Some(gold_remaining) = available_gold.checked_sub(cost) else {
        return Err(ForgeError::InsufficientGold {
            required: cost,
            available: available_gold,
        });
    };

    Ok(ForgeOutcome {
        consumed_items: FORGE_ITEM_COST,
        produced_quality,
        gold_spent: cost,
        gold_remaining,
    })
}

#[cfg(test)]
mod tests {
    use crate::economy::Gold;

    use super::{FORGE_ITEM_COST, ForgeError, forge, forge_cost};
    use crate::item::Quality;

    #[test]
    fn forge_consumes_four_items() {
        let outcome = forge(Quality::Standard, 4, Gold::new(100)).expect("forge should succeed");

        assert_eq!(outcome.consumed_items(), FORGE_ITEM_COST);
    }

    #[test]
    fn forge_produces_next_quality() {
        let outcome = forge(Quality::Rare, 4, Gold::new(1_000_000)).expect("forge should succeed");

        assert_eq!(outcome.produced_quality(), Quality::Epic);
    }

    #[test]
    fn forge_rejects_insufficient_items() {
        assert_eq!(
            forge(Quality::Standard, 3, Gold::new(100)),
            Err(ForgeError::InsufficientItems {
                required: 4,
                available: 3,
            })
        );
    }

    #[test]
    fn forge_rejects_insufficient_gold() {
        let cost = forge_cost(Quality::Rare).expect("rare should be forgeable");

        assert_eq!(
            forge(Quality::Rare, 4, Gold::ZERO),
            Err(ForgeError::InsufficientGold {
                required: cost,
                available: Gold::ZERO,
            })
        );
    }

    #[test]
    fn primordial_cannot_be_forged() {
        assert_eq!(forge_cost(Quality::Primordial), None);

        assert_eq!(
            forge(Quality::Primordial, 4, Gold::new(u64::MAX)),
            Err(ForgeError::FinalQuality)
        );
    }

    #[test]
    fn forge_cost_increases_with_quality() {
        let standard = forge_cost(Quality::Standard).expect("standard should be forgeable");
        let common = forge_cost(Quality::Common).expect("common should be forgeable");
        let uncommon = forge_cost(Quality::Uncommon).expect("uncommon should be forgeable");

        assert!(standard < common);
        assert!(common < uncommon);
    }

    #[test]
    fn forge_reports_remaining_gold() {
        let cost = forge_cost(Quality::Standard).expect("standard should be forgeable");
        let available = Gold::new(cost.amount() + 500);

        let outcome = forge(Quality::Standard, 4, available).expect("forge should succeed");

        assert_eq!(outcome.gold_spent(), cost);
        assert_eq!(outcome.gold_remaining(), Gold::new(500));
    }
}
