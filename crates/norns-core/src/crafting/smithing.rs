use crate::{
    item::{CraftedItem, ItemKind, ItemStack, Resource},
    progression::{Experience, SkillLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmithingRecipe {
    input: ItemStack,
    output: ItemStack,
    required_level: SkillLevel,
    smithing_xp: Experience,
    sword_specialization_xp: Experience,
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
        sword_specialization_xp: 20,
    }
};

/// Describes one completed craft; applying costs and rewards is the caller's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmithingOutcome {
    consumed: ItemStack,
    stack: ItemStack,
    smithing_xp: Experience,
    sword_specialization_xp: Experience,
}

impl SmithingOutcome {
    #[must_use]
    pub const fn consumed(self) -> ItemStack {
        self.consumed
    }

    #[must_use]
    pub const fn stack(self) -> ItemStack {
        self.stack
    }

    #[must_use]
    pub const fn smithing_xp(self) -> Experience {
        self.smithing_xp
    }

    #[must_use]
    pub const fn sword_specialization_xp(self) -> Experience {
        self.sword_specialization_xp
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
/// the resource cost and rewards together. This function does not roll quality.
#[must_use]
pub const fn smith(recipe: SmithingRecipe) -> SmithingOutcome {
    SmithingOutcome {
        consumed: recipe.input,
        stack: recipe.output,
        smithing_xp: recipe.smithing_xp,
        sword_specialization_xp: recipe.sword_specialization_xp,
    }
}

#[cfg(test)]
mod tests {
    use super::{IRON_SWORD, can_smith, smith};
    use crate::item::{CraftedItem, ItemKind, Resource};

    #[test]
    fn iron_sword_requires_smithing_level() {
        assert!(!can_smith(IRON_SWORD, IRON_SWORD.required_level() - 1));
        assert!(can_smith(IRON_SWORD, IRON_SWORD.required_level()));
        assert!(can_smith(IRON_SWORD, IRON_SWORD.required_level() + 1));
    }

    #[test]
    fn iron_sword_craft_describes_resource_cost_and_rewards() {
        let outcome = smith(IRON_SWORD);

        assert_eq!(outcome.consumed(), IRON_SWORD.input());
        assert_eq!(
            outcome.consumed().kind(),
            ItemKind::Resource(Resource::IronOre)
        );
        assert_eq!(outcome.consumed().quantity(), 2);
        assert_eq!(outcome.stack(), IRON_SWORD.output());
        assert_eq!(
            outcome.stack().kind(),
            ItemKind::CraftedItem(CraftedItem::IronSword)
        );
        assert_eq!(outcome.stack().quantity(), 1);
        assert_eq!(outcome.smithing_xp(), 20);
        assert_eq!(outcome.sword_specialization_xp(), 20);
    }
}
