use crate::{
    item::{
        CraftedItem, CraftedItemStack, ItemKind, ItemStack, QualityRoll, Resource, roll_quality,
    },
    progression::{Experience, SkillLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmithingRecipe {
    input: ItemStack,
    output: ItemStack,
    required_level: SkillLevel,
    smithing_xp: Experience,
    specialization_xp: Experience,
}

impl SmithingRecipe {
    #[must_use]
    pub const fn input(self) -> ItemStack {
        self.input
    }

    #[must_use]
    pub const fn output(self) -> ItemStack {
        self.output
    }

    #[must_use]
    pub const fn required_level(self) -> SkillLevel {
        self.required_level
    }
}

/// Provisional recipe: two Iron Ore become one Iron Sword.
///
/// Inputs, level requirement, and XP are development placeholders, including
/// direct use of ore rather than a future smelting/intermediate-material loop.
pub const IRON_SWORD: SmithingRecipe = {
    let Some(input) = ItemStack::new(ItemKind::Resource(Resource::IronOre), 2) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(ItemKind::CraftedItem(CraftedItem::IronSword), 1) else {
        unreachable!()
    };

    SmithingRecipe {
        input,
        output,
        required_level: 5,
        smithing_xp: 20,
        specialization_xp: 20,
    }
};

/// Describes one completed craft; applying costs and rewards is the caller's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmithingOutcome {
    consumed: ItemStack,
    produced: CraftedItemStack,
    smithing_xp: Experience,
    specialization_xp: Experience,
}

impl SmithingOutcome {
    #[must_use]
    pub const fn consumed(self) -> ItemStack {
        self.consumed
    }

    #[must_use]
    pub const fn produced(self) -> CraftedItemStack {
        self.produced
    }

    #[must_use]
    pub const fn smithing_xp(self) -> Experience {
        self.smithing_xp
    }

    #[must_use]
    pub const fn specialization_xp(self) -> Experience {
        self.specialization_xp
    }
}

/// Checks only the Smithing level requirement; callers also validate inputs.
#[must_use]
pub const fn can_smith(recipe: SmithingRecipe, smithing_level: SkillLevel) -> bool {
    smithing_level >= recipe.required_level
}

/// Describes the costs and rewards of one craft without mutating player state.
///
/// The activity layer must validate the level and available inputs, and apply
/// the resource cost and rewards together. Quality is resolved deterministically
/// from the server-supplied roll and the relevant crafting specialization level.
/// The activity layer maps the output identity to its specialization.
#[must_use]
pub fn smith(
    recipe: SmithingRecipe,
    quality_roll: QualityRoll,
    specialization_level: SkillLevel,
) -> SmithingOutcome {
    // Recipe fields are private; the defined recipes produce crafted items.
    let ItemKind::CraftedItem(item) = recipe.output.kind() else {
        unreachable!()
    };
    let quality = roll_quality(quality_roll, specialization_level);
    // The recipe's ItemStack already guarantees a positive quantity.
    let Some(produced) = CraftedItemStack::new(item, quality, recipe.output.quantity()) else {
        unreachable!()
    };

    SmithingOutcome {
        consumed: recipe.input,
        produced,
        smithing_xp: recipe.smithing_xp,
        specialization_xp: recipe.specialization_xp,
    }
}

#[cfg(test)]
mod tests {
    use super::{IRON_SWORD, can_smith, smith};
    use crate::item::{CraftedItem, ItemKind, Quality, QualityRoll, Resource};

    #[test]
    fn iron_sword_requires_smithing_level() {
        assert!(!can_smith(IRON_SWORD, IRON_SWORD.required_level() - 1));
        assert!(can_smith(IRON_SWORD, IRON_SWORD.required_level()));
        assert!(can_smith(IRON_SWORD, IRON_SWORD.required_level() + 1));
    }

    #[test]
    fn iron_sword_craft_describes_resource_cost_and_rewards() {
        let roll = QualityRoll::new(1_800_000).expect("roll should be valid");
        let outcome = smith(IRON_SWORD, roll, 1);

        assert_eq!(outcome.consumed(), IRON_SWORD.input());
        assert_eq!(
            outcome.consumed().kind(),
            ItemKind::Resource(Resource::IronOre)
        );
        assert_eq!(outcome.consumed().quantity(), 2);
        assert_eq!(
            ItemKind::CraftedItem(outcome.produced().item()),
            IRON_SWORD.output().kind()
        );
        assert_eq!(outcome.produced().item(), CraftedItem::IronSword);
        assert_eq!(outcome.produced().quality(), Quality::Rare);
        assert_eq!(
            outcome.produced().quantity(),
            IRON_SWORD.output().quantity()
        );
        assert_eq!(outcome.produced().quantity(), 1);
        assert_eq!(outcome.smithing_xp(), 20);
        assert_eq!(outcome.specialization_xp(), 20);
        assert_eq!(outcome, smith(IRON_SWORD, roll, 1));
    }

    #[test]
    fn specialization_improves_quality_without_changing_cost_or_xp() {
        let roll = QualityRoll::new(900_000).expect("roll should be valid");
        let baseline = smith(IRON_SWORD, roll, 1);
        let specialized = smith(IRON_SWORD, roll, 100);

        assert_eq!(baseline.produced().quality(), Quality::Standard);
        assert!(specialized.produced().quality() > baseline.produced().quality());
        assert_eq!(specialized.produced().item(), baseline.produced().item());
        assert_eq!(
            specialized.produced().quantity(),
            baseline.produced().quantity()
        );
        assert_eq!(specialized.consumed(), baseline.consumed());
        assert_eq!(specialized.smithing_xp(), baseline.smithing_xp());
        assert_eq!(
            specialized.specialization_xp(),
            baseline.specialization_xp()
        );
    }
}
