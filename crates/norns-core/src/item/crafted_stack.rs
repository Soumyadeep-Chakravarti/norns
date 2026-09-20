use super::{CraftedItem, Quality};

/// A positive quantity of crafted items with an explicitly resolved quality.
///
/// Unlike an identity-and-quantity description in [`super::ItemStack`], this
/// represents quality-resolved items regardless of their acquisition source.
/// Standard is a resolved quality, not a default for missing quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CraftedItemStack {
    item: CraftedItem,
    quality: Quality,
    quantity: u32,
}

impl CraftedItemStack {
    /// Returns `None` if the quantity is zero.
    #[must_use]
    pub const fn new(item: CraftedItem, quality: Quality, quantity: u32) -> Option<Self> {
        if quantity == 0 {
            None
        } else {
            Some(Self {
                item,
                quality,
                quantity,
            })
        }
    }

    #[must_use]
    pub const fn item(self) -> CraftedItem {
        self.item
    }

    #[must_use]
    pub const fn quality(self) -> Quality {
        self.quality
    }

    #[must_use]
    pub const fn quantity(self) -> u32 {
        self.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::{CraftedItem, CraftedItemStack, Quality};

    #[test]
    fn positive_quantities_preserve_identity_and_quality() {
        for quantity in [1, 4, u32::MAX] {
            let stack = CraftedItemStack::new(CraftedItem::IronSword, Quality::Rare, quantity)
                .expect("positive quantity should create a stack");

            assert_eq!(stack.item(), CraftedItem::IronSword);
            assert_eq!(stack.quality(), Quality::Rare);
            assert_eq!(stack.quantity(), quantity);
        }
    }

    #[test]
    fn zero_quantity_is_rejected() {
        assert_eq!(
            CraftedItemStack::new(CraftedItem::IronSword, Quality::Rare, 0),
            None
        );
    }

    #[test]
    fn different_qualities_are_unequal() {
        let rare = CraftedItemStack::new(CraftedItem::IronSword, Quality::Rare, 4)
            .expect("positive quantity should create a stack");
        let epic = CraftedItemStack::new(CraftedItem::IronSword, Quality::Epic, 4)
            .expect("positive quantity should create a stack");

        assert_ne!(rare, epic);
    }

    #[test]
    fn standard_is_an_explicit_resolved_quality() {
        let stack = CraftedItemStack::new(CraftedItem::IronSword, Quality::Standard, 1)
            .expect("positive quantity should create a stack");

        assert_eq!(stack.quality(), Quality::Standard);
    }
}
