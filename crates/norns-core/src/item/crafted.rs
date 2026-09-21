use super::MaterialTier;

/// Crafted-item identities, distinct from refined materials and cooked items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CraftedItem {
    Axe(MaterialTier),
    Sword(MaterialTier),
}
