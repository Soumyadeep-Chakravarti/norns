#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SkillProgress {
    xp: u64,
}

impl SkillProgress {
    #[must_use]
    pub const fn new() -> Self {
        Self { xp: 0 }
    }

    #[must_use]
    pub const fn xp(self) -> u64 {
        self.xp
    }

    pub const fn add_xp(&mut self, amount: u64) {
        self.xp = self.xp.saturating_add(amount);
    }
}
