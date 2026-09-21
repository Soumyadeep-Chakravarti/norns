use crate::{
    item::{
        CraftedItem, CraftedItemStack, ItemKind, ItemStack, MaterialTier, QualityRoll,
        RefinedMaterial, roll_quality,
    },
    progression::{Experience, SkillLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Deterministic recipe for producing a quality-bearing crafted item.
pub struct SmithingRecipe {
    input: ItemStack,
    output: ItemStack,
    required_level: SkillLevel,
    smithing_xp: Experience,
    specialization_xp: Experience,
}

impl SmithingRecipe {
    #[must_use]
    /// Creates a sword recipe for a material tier.
    pub const fn sword_for_tier(tier: MaterialTier) -> Self {
        Self::for_item(tier, CraftedItem::Sword(tier), 2)
    }

    #[must_use]
    /// Creates an axe recipe for a material tier.
    pub const fn axe_for_tier(tier: MaterialTier) -> Self {
        Self::for_item(tier, CraftedItem::Axe(tier), 3)
    }

    const fn for_item(tier: MaterialTier, item: CraftedItem, input_quantity: u32) -> Self {
        let tier_number = tier.tier() as u32;
        let Some(input) = ItemStack::new(
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(tier)),
            input_quantity,
        ) else {
            unreachable!()
        };
        let Some(output) = ItemStack::new(ItemKind::CraftedItem(item), 1) else {
            unreachable!()
        };

        Self {
            input,
            output,
            required_level: tier_number.saturating_mul(5),
            smithing_xp: (tier_number as u64).saturating_mul(20),
            specialization_xp: (tier_number as u64).saturating_mul(20),
        }
    }

    #[must_use]
    /// Returns the ingot input stack.
    pub const fn input(self) -> ItemStack {
        self.input
    }

    #[must_use]
    /// Returns the unresolved crafted output stack.
    pub const fn output(self) -> ItemStack {
        self.output
    }

    #[must_use]
    /// Returns the required Smithing level.
    pub const fn required_level(self) -> SkillLevel {
        self.required_level
    }
}

/// Provisional recipe: two Iron Ingots become one Iron Sword.
///
/// Inputs, level requirement, and XP remain development placeholders.
pub const IRON_SWORD: SmithingRecipe = {
    let Some(input) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron)),
        2,
    ) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Iron)),
        1,
    ) else {
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

/// Provisional copper sword recipe.
pub const COPPER_SWORD: SmithingRecipe = {
    let Some(input) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Copper)),
        2,
    ) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Copper)),
        1,
    ) else {
        unreachable!()
    };

    SmithingRecipe {
        input,
        output,
        required_level: 1,
        smithing_xp: 10,
        specialization_xp: 10,
    }
};

/// Provisional tin sword recipe.
pub const TIN_SWORD: SmithingRecipe = {
    let Some(input) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Tin)),
        2,
    ) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Tin)),
        1,
    ) else {
        unreachable!()
    };

    SmithingRecipe {
        input,
        output,
        required_level: 3,
        smithing_xp: 15,
        specialization_xp: 15,
    }
};

/// Provisional copper axe recipe.
pub const COPPER_AXE: SmithingRecipe = {
    let Some(input) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Copper)),
        3,
    ) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Copper)),
        1,
    ) else {
        unreachable!()
    };

    SmithingRecipe {
        input,
        output,
        required_level: 2,
        smithing_xp: 12,
        specialization_xp: 12,
    }
};

/// Provisional tin axe recipe.
pub const TIN_AXE: SmithingRecipe = {
    let Some(input) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Tin)),
        3,
    ) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Tin)),
        1,
    ) else {
        unreachable!()
    };

    SmithingRecipe {
        input,
        output,
        required_level: 4,
        smithing_xp: 18,
        specialization_xp: 18,
    }
};

/// Provisional iron axe recipe.
pub const IRON_AXE: SmithingRecipe = {
    let Some(input) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron)),
        3,
    ) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Iron)),
        1,
    ) else {
        unreachable!()
    };

    SmithingRecipe {
        input,
        output,
        required_level: 6,
        smithing_xp: 24,
        specialization_xp: 24,
    }
};

/// Describes one completed craft; applying costs and rewards is the caller's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of one completed Smithing activity.
pub struct SmithingOutcome {
    consumed: ItemStack,
    produced: CraftedItemStack,
    smithing_xp: Experience,
    specialization_xp: Experience,
}

impl SmithingOutcome {
    #[must_use]
    /// Returns the consumed ingot stack.
    pub const fn consumed(self) -> ItemStack {
        self.consumed
    }

    #[must_use]
    /// Returns the quality-resolved crafted output.
    pub const fn produced(self) -> CraftedItemStack {
        self.produced
    }

    #[must_use]
    /// Returns broad Smithing XP.
    pub const fn smithing_xp(self) -> Experience {
        self.smithing_xp
    }

    #[must_use]
    /// Returns Smithing specialization XP.
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
    use super::{
        COPPER_AXE, COPPER_SWORD, IRON_AXE, IRON_SWORD, SmithingRecipe, TIN_AXE, TIN_SWORD,
        can_smith, smith,
    };
    use crate::item::{CraftedItem, ItemKind, MaterialTier, Quality, QualityRoll, RefinedMaterial};

    #[test]
    fn material_swords_use_their_matching_ingots() {
        assert_eq!(
            COPPER_SWORD.input().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Copper))
        );
        assert_eq!(
            TIN_SWORD.input().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Tin))
        );
        assert_eq!(
            COPPER_SWORD.output().kind(),
            ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Copper))
        );
        assert_eq!(
            TIN_SWORD.output().kind(),
            ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Tin))
        );
    }

    #[test]
    fn material_axes_use_their_matching_ingots() {
        assert_eq!(
            COPPER_AXE.input().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Copper))
        );
        assert_eq!(
            TIN_AXE.input().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Tin))
        );
        assert_eq!(
            IRON_AXE.input().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron))
        );
        assert_eq!(
            COPPER_AXE.output().kind(),
            ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Copper))
        );
        assert_eq!(
            TIN_AXE.output().kind(),
            ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Tin))
        );
        assert_eq!(
            IRON_AXE.output().kind(),
            ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Iron))
        );
    }

    #[test]
    fn iron_sword_requires_smithing_level() {
        assert!(!can_smith(IRON_SWORD, IRON_SWORD.required_level() - 1));
        assert!(can_smith(IRON_SWORD, IRON_SWORD.required_level()));
        assert!(can_smith(IRON_SWORD, IRON_SWORD.required_level() + 1));
    }

    #[test]
    fn iron_sword_craft_describes_refined_material_cost_and_rewards() {
        let roll = QualityRoll::new(1_800_000).expect("roll should be valid");
        let outcome = smith(IRON_SWORD, roll, 1);

        assert_eq!(outcome.consumed(), IRON_SWORD.input());
        assert_eq!(
            outcome.consumed().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron))
        );
        assert_eq!(outcome.consumed().quantity(), 2);
        assert_eq!(
            ItemKind::CraftedItem(outcome.produced().item()),
            IRON_SWORD.output().kind()
        );
        assert_eq!(
            outcome.produced().item(),
            CraftedItem::Sword(MaterialTier::Iron)
        );
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

    #[test]
    fn every_material_tier_has_sword_and_axe_recipes() {
        for tier in MaterialTier::ALL {
            let sword = SmithingRecipe::sword_for_tier(tier);
            let axe = SmithingRecipe::axe_for_tier(tier);

            assert_eq!(
                sword.input().kind(),
                ItemKind::RefinedMaterial(RefinedMaterial::Ingot(tier))
            );
            assert_eq!(
                sword.output().kind(),
                ItemKind::CraftedItem(CraftedItem::Sword(tier))
            );
            assert_eq!(
                axe.output().kind(),
                ItemKind::CraftedItem(CraftedItem::Axe(tier))
            );
            assert_eq!(sword.output().quantity(), 1);
            assert_eq!(axe.output().quantity(), 1);
        }
    }
}
