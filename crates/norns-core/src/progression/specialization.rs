use super::{Experience, SkillLevel, level_from_xp};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SpecializationProgress {
    xp: Experience,
}

impl SpecializationProgress {
    #[must_use]
    pub const fn new() -> Self {
        Self { xp: 0 }
    }

    #[must_use]
    pub const fn xp(self) -> Experience {
        self.xp
    }

    #[must_use]
    pub fn level(self) -> SkillLevel {
        level_from_xp(self.xp)
    }

    pub const fn add_xp(&mut self, amount: Experience) {
        self.xp = self.xp.saturating_add(amount);
    }
}

#[cfg(test)]
mod tests {
    use super::SpecializationProgress;

    #[test]
    fn experience_accumulates() {
        let mut progress = SpecializationProgress::new();

        progress.add_xp(100);
        progress.add_xp(50);

        assert_eq!(progress.xp(), 150);
    }

    #[test]
    fn experience_saturates() {
        let mut progress = SpecializationProgress::new();

        progress.add_xp(u64::MAX);
        progress.add_xp(1);

        assert_eq!(progress.xp(), u64::MAX);
    }
}
