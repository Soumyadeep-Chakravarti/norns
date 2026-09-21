#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The authoritative in-game gold balance.
pub struct Gold(u64);

impl Gold {
    /// A zero balance.
    pub const ZERO: Self = Self(0);

    #[must_use]
    /// Creates a balance from an amount of gold.
    pub const fn new(amount: u64) -> Self {
        Self(amount)
    }

    #[must_use]
    /// Returns the raw gold amount.
    pub const fn amount(self) -> u64 {
        self.0
    }

    #[must_use]
    /// Returns whether this balance covers `cost`.
    pub const fn can_afford(self, cost: Self) -> bool {
        self.0 >= cost.0
    }

    #[must_use]
    /// Adds gold without allowing integer overflow.
    pub const fn saturating_add(self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    #[must_use]
    /// Subtracts gold, returning `None` when the balance is insufficient.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(amount) => Some(Self(amount)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Gold;

    #[test]
    fn gold_reports_its_amount() {
        assert_eq!(Gold::new(500).amount(), 500);
    }

    #[test]
    fn affordability_is_checked() {
        assert!(Gold::new(500).can_afford(Gold::new(500)));
        assert!(Gold::new(600).can_afford(Gold::new(500)));
        assert!(!Gold::new(499).can_afford(Gold::new(500)));
    }

    #[test]
    fn subtraction_fails_when_gold_is_insufficient() {
        assert_eq!(Gold::new(100).checked_sub(Gold::new(101)), None);
    }

    #[test]
    fn subtraction_returns_remaining_gold() {
        assert_eq!(
            Gold::new(500).checked_sub(Gold::new(200)),
            Some(Gold::new(300))
        );
    }

    #[test]
    fn addition_saturates() {
        assert_eq!(
            Gold::new(u64::MAX).saturating_add(Gold::new(1)),
            Gold::new(u64::MAX)
        );
    }
}
