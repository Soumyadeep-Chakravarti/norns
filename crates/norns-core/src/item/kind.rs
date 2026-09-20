use super::{CraftedItem, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Resource(Resource),
    CraftedItem(CraftedItem),
}

impl ItemKind {
    /// Quality eligibility is intrinsic to the category, regardless of source.
    #[must_use]
    pub const fn supports_quality(self) -> bool {
        matches!(self, Self::CraftedItem(_))
    }
}

#[cfg(test)]
mod tests {
    use super::{CraftedItem, ItemKind, Resource};

    #[test]
    fn raw_resources_do_not_support_quality() {
        for resource in [
            Resource::Stone,
            Resource::CopperOre,
            Resource::TinOre,
            Resource::IronOre,
        ] {
            assert!(!ItemKind::Resource(resource).supports_quality());
        }
    }

    #[test]
    fn crafted_items_support_quality() {
        assert!(ItemKind::CraftedItem(CraftedItem::IronSword).supports_quality());
    }
}
