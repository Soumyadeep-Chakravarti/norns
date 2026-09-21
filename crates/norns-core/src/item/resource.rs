use super::MaterialTier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Stone,
    CopperOre,
    TinOre,
    IronOre,
    TieredOre(MaterialTier),
}
