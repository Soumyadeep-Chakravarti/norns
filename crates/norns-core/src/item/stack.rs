use super::ItemKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// An unresolved item identity and positive quantity.
pub struct ItemStack {
    kind: ItemKind,
    quantity: u32,
}

impl ItemStack {
    #[must_use]
    /// Creates a stack, returning `None` for zero quantity.
    pub const fn new(kind: ItemKind, quantity: u32) -> Option<Self> {
        if quantity == 0 {
            None
        } else {
            Some(Self { kind, quantity })
        }
    }

    #[must_use]
    /// Returns the item identity.
    pub const fn kind(self) -> ItemKind {
        self.kind
    }

    #[must_use]
    /// Returns the number of items in the stack.
    pub const fn quantity(self) -> u32 {
        self.quantity
    }
}

#[cfg(test)]
mod tests {
    use crate::item::{ItemKind, Resource};

    use super::ItemStack;

    #[test]
    fn item_stack_preserves_identity_and_quantity() {
        let stack = ItemStack::new(ItemKind::Resource(Resource::CopperOre), 17)
            .expect("positive quantity should create a stack");

        assert_eq!(stack.kind(), ItemKind::Resource(Resource::CopperOre));
        assert_eq!(stack.quantity(), 17);
    }

    #[test]
    fn item_stack_rejects_zero_quantity() {
        assert_eq!(ItemStack::new(ItemKind::Resource(Resource::Stone), 0), None);
    }
}
