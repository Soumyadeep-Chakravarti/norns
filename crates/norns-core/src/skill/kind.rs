use super::SkillTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillKind {
    // Gathering
    Mining,
    Woodcutting,
    Fishing,
    Foraging,
    Hunting,

    // Crafting
    Smithing,
    Woodworking,
    Cooking,
    Leatherworking,
    Alchemy,
    Runecrafting,

    // Combat
    Melee,
    Ranged,
    Seidr,
}

impl SkillKind {
    #[must_use]
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
