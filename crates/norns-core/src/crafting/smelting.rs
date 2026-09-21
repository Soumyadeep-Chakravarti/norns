use crate::{
    item::{ItemKind, ItemStack, MaterialTier, RefinedMaterial, Resource},
    progression::{Experience, SkillLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmeltingRecipe {
    input: ItemStack,
    output: ItemStack,
    required_level: SkillLevel,
    smithing_xp: Experience,
    specialization_xp: Experience,
}

impl SmeltingRecipe {
    #[must_use]
    pub const fn for_tier(tier: MaterialTier) -> Self {
        let tier_number = tier.tier() as u32;
        let Some(input) = ItemStack::new(ItemKind::Resource(Resource::TieredOre(tier)), 2) else {
            unreachable!()
        };
        let Some(output) =
            ItemStack::new(ItemKind::RefinedMaterial(RefinedMaterial::Ingot(tier)), 1)
        else {
            unreachable!()
        };

        Self {
            input,
            output,
            required_level: tier_number.saturating_mul(5),
            smithing_xp: (tier_number as u64).saturating_mul(10),
            specialization_xp: (tier_number as u64).saturating_mul(10),
        }
    }

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

/// Provisional recipe: two Iron Ore become one Iron Ingot.
pub const IRON_INGOT: SmeltingRecipe = {
    let Some(input) = ItemStack::new(ItemKind::Resource(Resource::IronOre), 2) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron)),
        1,
    ) else {
        unreachable!()
    };

    SmeltingRecipe {
        input,
        output,
        required_level: 3,
        smithing_xp: 10,
        specialization_xp: 10,
    }
};

pub const COPPER_INGOT: SmeltingRecipe = {
    let Some(input) = ItemStack::new(ItemKind::Resource(Resource::CopperOre), 2) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Copper)),
        1,
    ) else {
        unreachable!()
    };

    SmeltingRecipe {
        input,
        output,
        required_level: 1,
        smithing_xp: 5,
        specialization_xp: 5,
    }
};

pub const TIN_INGOT: SmeltingRecipe = {
    let Some(input) = ItemStack::new(ItemKind::Resource(Resource::TinOre), 2) else {
        unreachable!()
    };
    let Some(output) = ItemStack::new(
        ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Tin)),
        1,
    ) else {
        unreachable!()
    };

    SmeltingRecipe {
        input,
        output,
        required_level: 2,
        smithing_xp: 8,
        specialization_xp: 8,
    }
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmeltingOutcome {
    consumed: ItemStack,
    produced: ItemStack,
    smithing_xp: Experience,
    specialization_xp: Experience,
}

impl SmeltingOutcome {
    #[must_use]
    pub const fn consumed(self) -> ItemStack {
        self.consumed
    }

    #[must_use]
    pub const fn produced(self) -> ItemStack {
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

#[must_use]
pub const fn can_smelt(recipe: SmeltingRecipe, smithing_level: SkillLevel) -> bool {
    smithing_level >= recipe.required_level
}

/// Describes one completed smelting activity without mutating player state.
#[must_use]
pub const fn smelt(recipe: SmeltingRecipe) -> SmeltingOutcome {
    SmeltingOutcome {
        consumed: recipe.input,
        produced: recipe.output,
        smithing_xp: recipe.smithing_xp,
        specialization_xp: recipe.specialization_xp,
    }
}

#[cfg(test)]
mod tests {
    use crate::item::{ItemKind, MaterialTier, RefinedMaterial, Resource};

    use super::{COPPER_INGOT, IRON_INGOT, SmeltingRecipe, TIN_INGOT, can_smelt, smelt};

    #[test]
    fn iron_ingot_requires_smithing_level() {
        assert!(!can_smelt(IRON_INGOT, 2));
        assert!(can_smelt(IRON_INGOT, 3));
    }

    #[test]
    fn ingot_recipes_match_their_ore() {
        assert_eq!(
            COPPER_INGOT.input().kind(),
            ItemKind::Resource(Resource::CopperOre)
        );
        assert_eq!(
            TIN_INGOT.input().kind(),
            ItemKind::Resource(Resource::TinOre)
        );
        assert_eq!(
            IRON_INGOT.input().kind(),
            ItemKind::Resource(Resource::IronOre)
        );
        assert_eq!(
            COPPER_INGOT.output().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Copper))
        );
        assert_eq!(
            TIN_INGOT.output().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Tin))
        );
        assert_eq!(
            IRON_INGOT.output().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron))
        );
    }

    #[test]
    fn iron_ingot_consumes_ore_without_quality() {
        let outcome = smelt(IRON_INGOT);

        assert_eq!(
            outcome.consumed().kind(),
            ItemKind::Resource(Resource::IronOre)
        );
        assert_eq!(outcome.consumed().quantity(), 2);
        assert_eq!(
            outcome.produced().kind(),
            ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron))
        );
        assert_eq!(outcome.produced().quantity(), 1);
        assert_eq!(outcome.smithing_xp(), 10);
        assert_eq!(outcome.specialization_xp(), 10);
    }

    #[test]
    fn every_material_tier_has_a_smelting_recipe() {
        for tier in MaterialTier::ALL {
            let recipe = SmeltingRecipe::for_tier(tier);
            assert_eq!(
                recipe.input().kind(),
                ItemKind::Resource(Resource::TieredOre(tier))
            );
            assert_eq!(
                recipe.output().kind(),
                ItemKind::RefinedMaterial(RefinedMaterial::Ingot(tier))
            );
            assert_eq!(recipe.input().quantity(), 2);
            assert_eq!(recipe.output().quantity(), 1);
        }
    }
}
