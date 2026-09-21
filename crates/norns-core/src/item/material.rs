/// Material tiers shared by refined metal and metal equipment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// Norse/Valheim-inspired material progression tier.
pub enum MaterialTier {
    /// Early copper tier.
    Copper,
    /// Early tin tier.
    Tin,
    /// Copper-and-tin alloy tier.
    Bronze,
    /// Iron tier.
    Iron,
    /// Refined steel tier.
    Steel,
    /// Precious silver tier.
    Silver,
    /// Blackmetal tier.
    Blackmetal,
    /// Volcanic obsidian tier.
    Obsidian,
    /// Ancient root tier.
    Root,
    /// Fenris-themed tier.
    Fenris,
    /// Carapace tier.
    Carapace,
    /// Eitr-infused tier.
    Eitr,
    /// Dvergr-forged tier.
    Dvergr,
    /// Jotunn tier.
    Jotunn,
    /// Aesir tier.
    Aesir,
    /// Bifrost tier.
    Bifrost,
    /// Yggdrasil tier.
    Yggdrasil,
    /// Final Norn tier.
    Norn,
}

impl MaterialTier {
    /// All material tiers in progression order.
    pub const ALL: [Self; 18] = [
        Self::Copper,
        Self::Tin,
        Self::Bronze,
        Self::Iron,
        Self::Steel,
        Self::Silver,
        Self::Blackmetal,
        Self::Obsidian,
        Self::Root,
        Self::Fenris,
        Self::Carapace,
        Self::Eitr,
        Self::Dvergr,
        Self::Jotunn,
        Self::Aesir,
        Self::Bifrost,
        Self::Yggdrasil,
        Self::Norn,
    ];

    #[must_use]
    /// Returns the one-based numeric tier.
    pub const fn tier(self) -> u8 {
        match self {
            Self::Copper => 1,
            Self::Tin => 2,
            Self::Bronze => 3,
            Self::Iron => 4,
            Self::Steel => 5,
            Self::Silver => 6,
            Self::Blackmetal => 7,
            Self::Obsidian => 8,
            Self::Root => 9,
            Self::Fenris => 10,
            Self::Carapace => 11,
            Self::Eitr => 12,
            Self::Dvergr => 13,
            Self::Jotunn => 14,
            Self::Aesir => 15,
            Self::Bifrost => 16,
            Self::Yggdrasil => 17,
            Self::Norn => 18,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MaterialTier;

    #[test]
    fn material_tiers_are_ordered_from_one_to_eighteen() {
        for (index, tier) in MaterialTier::ALL.into_iter().enumerate() {
            assert_eq!(tier.tier(), u8::try_from(index + 1).unwrap());
        }
    }
}
