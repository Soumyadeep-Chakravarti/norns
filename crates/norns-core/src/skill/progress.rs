use crate::progression::{Experience, SkillLevel, level_from_xp};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
/// XP and level state for a broad skill.
pub struct SkillProgress {
    xp: Experience,
}

impl SkillProgress {
    #[must_use]
    /// Creates a level-one skill with no XP.
    pub const fn new() -> Self {
        Self { xp: 0 }
    }

    #[must_use]
    /// Returns accumulated skill XP.
    pub const fn xp(self) -> Experience {
        self.xp
    }

    #[must_use]
    /// Returns the current skill level.
    pub fn level(self) -> SkillLevel {
        level_from_xp(self.xp)
    }

    /// Adds XP using saturating arithmetic.
    pub const fn add_xp(&mut self, amount: Experience) {
        self.xp = self.xp.saturating_add(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::SkillProgress;

    #[test]
    fn experience_accumulates() {
        let mut progress = SkillProgress::new();

        progress.add_xp(100);
        progress.add_xp(50);

        assert_eq!(progress.xp(), 150);
    }

    #[test]
    fn experience_saturates() {
        let mut progress = SkillProgress::new();

        progress.add_xp(u64::MAX);
        progress.add_xp(1);

        assert_eq!(progress.xp(), u64::MAX);
    }
}
