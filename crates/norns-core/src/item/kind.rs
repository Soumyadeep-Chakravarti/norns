use super::{CraftedItem, RefinedMaterial, Resource};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Resource(Resource),
    RefinedMaterial(RefinedMaterial),
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
    use super::{CraftedItem, ItemKind, RefinedMaterial, Resource};
    use crate::item::MaterialTier;

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
    fn refined_materials_do_not_support_quality() {
        assert!(
            !ItemKind::RefinedMaterial(RefinedMaterial::Ingot(MaterialTier::Iron))
                .supports_quality()
        );
    }

    #[test]
    fn crafted_items_support_quality() {
        assert!(ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Copper)).supports_quality());
        assert!(ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Copper)).supports_quality());
        assert!(ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Iron)).supports_quality());
        assert!(ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Iron)).supports_quality());
        assert!(ItemKind::CraftedItem(CraftedItem::Axe(MaterialTier::Tin)).supports_quality());
        assert!(ItemKind::CraftedItem(CraftedItem::Sword(MaterialTier::Tin)).supports_quality());
    }
}
