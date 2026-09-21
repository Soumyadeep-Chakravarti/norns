use super::MaterialTier;

/// Crafted-item identities, distinct from refined materials and cooked items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CraftedItem {
    /// A melee axe at the specified material tier.
    Axe(MaterialTier),
    /// A melee sword at the specified material tier.
    Sword(MaterialTier),
}
