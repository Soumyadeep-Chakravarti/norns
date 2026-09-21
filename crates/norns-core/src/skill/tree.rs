#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Broad category containing related skills.
pub enum SkillTree {
    /// Resource extraction skills.
    Gathering,
    /// Item production skills.
    Crafting,
    /// Player and creature combat skills.
    Combat,
}
