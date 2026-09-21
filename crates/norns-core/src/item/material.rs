/// Material tiers shared by refined metal and metal equipment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MaterialTier {
    Copper,
    Tin,
    Bronze,
    Iron,
    Steel,
    Silver,
    Blackmetal,
    Obsidian,
    Root,
    Fenris,
    Carapace,
    Eitr,
    Dvergr,
    Jotunn,
    Aesir,
    Bifrost,
    Yggdrasil,
    Norn,
}

impl MaterialTier {
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
