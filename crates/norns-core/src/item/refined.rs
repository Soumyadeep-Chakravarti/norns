use super::MaterialTier;

/// Refined material identities, distinct from raw resources and crafted items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefinedMaterial {
    Ingot(MaterialTier),
}
