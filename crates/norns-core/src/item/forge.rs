use crate::economy::Gold;

use super::{CraftedItemStack, Quality};

/// Number of identical items consumed by one Forge upgrade.
pub const FORGE_ITEM_COST: u32 = 4;

const BASE_FORGE_COST: u64 = 100;
const FORGE_COST_MULTIPLIER: u64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of a successful deterministic Forge upgrade.
pub struct ForgeOutcome {
    consumed_items: u32,
    produced: CraftedItemStack,
    gold_spent: Gold,
    gold_remaining: Gold,
}

impl ForgeOutcome {
    #[must_use]
    /// Returns the number of input items consumed.
    pub const fn consumed_items(self) -> u32 {
        self.consumed_items
    }

    #[must_use]
    /// Returns the upgraded crafted item.
    pub const fn produced(self) -> CraftedItemStack {
        self.produced
    }

    #[must_use]
    /// Returns the Forge fee.
    pub const fn gold_spent(self) -> Gold {
        self.gold_spent
    }

    #[must_use]
    /// Returns gold remaining after the fee.
    pub const fn gold_remaining(self) -> Gold {
        self.gold_remaining
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Failure reasons for a Forge operation.
pub enum ForgeError {
    /// The input is already at the final quality.
    FinalQuality,
    /// Fewer than four matching items were supplied.
    InsufficientItems {
        /// Number of items required by the Forge.
        required: u32,
        /// Number of items supplied by the input stack.
        available: u32,
    },
    /// The available gold did not cover the fee.
    InsufficientGold {
        /// Gold required by the Forge.
        required: Gold,
        /// Gold supplied by the player.
        available: Gold,
    },
}

#[must_use]
/// Returns the gold cost to advance one quality tier.
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

/// Describes forging four identical crafted items into one at the next quality.
///
/// The input stack supplies a single item identity and quality. The activity
/// layer applies the consumed count, produced stack, and gold cost to inventory;
/// this operation does not mutate inventory or return leftover input items.
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
    stack: CraftedItemStack,
    available_gold: Gold,
) -> Result<ForgeOutcome, ForgeError> {
    let Some(produced_quality) = stack.quality().next() else {
        return Err(ForgeError::FinalQuality);
    };

    if stack.quantity() < FORGE_ITEM_COST {
        return Err(ForgeError::InsufficientItems {
            required: FORGE_ITEM_COST,
            available: stack.quantity(),
        });
    }

    let Some(cost) = forge_cost(stack.quality()) else {
        return Err(ForgeError::FinalQuality);
    };

    let Some(gold_remaining) = available_gold.checked_sub(cost) else {
        return Err(ForgeError::InsufficientGold {
            required: cost,
            available: available_gold,
        });
    };

    let Some(produced) = CraftedItemStack::new(stack.item(), produced_quality, 1) else {
        unreachable!()
    };

    Ok(ForgeOutcome {
        consumed_items: FORGE_ITEM_COST,
        produced,
        gold_spent: cost,
        gold_remaining,
    })
}

#[cfg(test)]
mod tests {
    use crate::economy::Gold;

    use super::{FORGE_ITEM_COST, ForgeError, forge, forge_cost};
    use crate::item::{CraftedItem, CraftedItemStack, MaterialTier, Quality};

    fn swords(quality: Quality, quantity: u32) -> CraftedItemStack {
        CraftedItemStack::new(CraftedItem::Sword(MaterialTier::Iron), quality, quantity)
            .expect("positive quantity should create a stack")
    }

    #[test]
    fn forge_consumes_four_items() {
        let outcome =
            forge(swords(Quality::Standard, 4), Gold::new(100)).expect("forge should succeed");

        assert_eq!(outcome.consumed_items(), FORGE_ITEM_COST);
    }

    #[test]
    fn forge_produces_next_quality() {
        let outcome =
            forge(swords(Quality::Rare, 7), Gold::new(1_000_000)).expect("forge should succeed");

        assert_eq!(outcome.consumed_items(), 4);
        assert_eq!(
            outcome.produced().item(),
            CraftedItem::Sword(MaterialTier::Iron)
        );
        assert_eq!(outcome.produced().quality(), Quality::Epic);
        assert_eq!(outcome.produced().quantity(), 1);
    }

    #[test]
    fn forge_rejects_insufficient_items() {
        assert_eq!(
            forge(swords(Quality::Standard, 3), Gold::new(100)),
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
            forge(swords(Quality::Rare, 4), Gold::ZERO),
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
            forge(swords(Quality::Primordial, 4), Gold::new(u64::MAX)),
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

        let outcome = forge(swords(Quality::Standard, 4), available).expect("forge should succeed");

        assert_eq!(outcome.gold_spent(), cost);
        assert_eq!(outcome.gold_remaining(), Gold::new(500));
    }
}
