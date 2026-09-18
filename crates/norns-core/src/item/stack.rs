use super::{ItemKind, Quality};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack {
    kind: ItemKind,
    quality: Quality,
    quantity: u32,
}

impl ItemStack {
    #[must_use]
    pub const fn new(kind: ItemKind, quality: Quality, quantity: u32) -> Option<Self> {
        if quantity == 0 {
            None
        } else {
            Some(Self {
                kind,
                quality,
                quantity,
            })
        }
    }

    #[must_use]
    pub const fn kind(self) -> ItemKind {
        self.kind
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
    use crate::item::{ItemKind, Quality, Resource};

    use super::ItemStack;

    #[test]
    fn item_stack_preserves_identity_quality_and_quantity() {
        let stack = ItemStack::new(ItemKind::Resource(Resource::CopperOre), Quality::Rare, 17)
            .expect("positive quantity should create a stack");

        assert_eq!(stack.kind(), ItemKind::Resource(Resource::CopperOre));
        assert_eq!(stack.quality(), Quality::Rare);
        assert_eq!(stack.quantity(), 17);
    }

    #[test]
    fn item_stack_rejects_zero_quantity() {
        assert_eq!(
            ItemStack::new(ItemKind::Resource(Resource::Stone), Quality::Standard, 0),
            None
        );
    }
}
