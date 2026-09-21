/// Player-supplied deterministic input for one attack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttackInput {
    critical: bool,
}

impl AttackInput {
    /// Creates an attack input with an explicit critical-hit result.
    #[must_use]
    pub const fn new(critical: bool) -> Self {
        Self { critical }
    }
    /// Returns whether this attack is critical.
    #[must_use]
    pub const fn critical(self) -> bool {
        self.critical
    }
}
