use super::SkillTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A broad skill or combat discipline.
pub enum SkillKind {
    // Gathering
    /// Extracts ore and stone.
    Mining,
    /// Extracts wood.
    Woodcutting,
    /// Catches fish.
    Fishing,
    /// Collects plants and natural ingredients.
    Foraging,
    /// Hunts wildlife for materials.
    Hunting,

    // Crafting
    /// Produces metal equipment and refined metal.
    Smithing,
    /// Produces wooden equipment and materials.
    Woodworking,
    /// Produces food.
    Cooking,
    /// Produces leather equipment and materials.
    Leatherworking,
    /// Produces potions and other mixtures.
    Alchemy,
    /// Produces runes and glyphs.
    Runecrafting,

    // Combat
    /// Uses melee weapons and physical techniques.
    Melee,
    /// Uses ranged weapons.
    Ranged,
    /// Uses Norns' magic discipline.
    Seidr,
}

impl SkillKind {
    #[must_use]
    /// Returns the broad tree containing this skill.
    pub const fn tree(self) -> SkillTree {
        match self {
            Self::Mining | Self::Woodcutting | Self::Fishing | Self::Foraging | Self::Hunting => {
                SkillTree::Gathering
            }

            Self::Smithing
            | Self::Woodworking
            | Self::Cooking
            | Self::Leatherworking
            | Self::Alchemy
            | Self::Runecrafting => SkillTree::Crafting,

            Self::Melee | Self::Ranged | Self::Seidr => SkillTree::Combat,
        }
    }
}
