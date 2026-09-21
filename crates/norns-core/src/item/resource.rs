use super::MaterialTier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Raw materials obtained from gathering activities.
pub enum Resource {
    /// Basic construction stone.
    Stone,
    /// Early copper ore.
    CopperOre,
    /// Early tin ore.
    TinOre,
    /// Early iron ore.
    IronOre,
    /// Ore associated with one of the material tiers.
    TieredOre(MaterialTier),
}
