#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Quality {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Ancient,
    Relic,
    Runebound,
    Cursed,
    Eldritch,
    Forgotten,
    Fatebound,
    HelForged,
    Jotunnforged,
    Voidborn,
    Einherjar,
    AesirTouched,
    Worldforged,
    NornTouched,
    Primordial,
}

impl Quality {
    #[must_use]
    pub const fn base_rarity(self) -> u64 {
        match self {
            Self::Common => 2,
            Self::Uncommon => 5,
            Self::Rare => 10,
            Self::Epic => 17,
            Self::Legendary => 33,
            Self::Mythic => 67,
            Self::Ancient => 143,
            Self::Relic => 333,
            Self::Runebound => 1_000,
            Self::Cursed => 2_000,
            Self::Eldritch => 5_000,
            Self::Forgotten => 10_000,
            Self::Fatebound => 20_000,
            Self::HelForged => 50_000,
            Self::Jotunnforged => 100_000,
            Self::Voidborn => 200_000,
            Self::Einherjar => 500_000,
            Self::AesirTouched => 667_000,
            Self::Worldforged => 833_000,
            Self::NornTouched => 1_000_000,
            Self::Primordial => 2_000_000,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
            Self::Legendary => "Legendary",
            Self::Mythic => "Mythic",
            Self::Ancient => "Ancient",
            Self::Relic => "Relic",
            Self::Runebound => "Runebound",
            Self::Cursed => "Cursed",
            Self::Eldritch => "Eldritch",
            Self::Forgotten => "Forgotten",
            Self::Fatebound => "Fatebound",
            Self::HelForged => "Hel-Forged",
            Self::Jotunnforged => "Jötunnforged",
            Self::Voidborn => "Voidborn",
            Self::Einherjar => "Einherjar",
            Self::AesirTouched => "Æsir-Touched",
            Self::Worldforged => "Worldforged",
            Self::NornTouched => "Norn-Touched",
            Self::Primordial => "Primordial",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Quality;

    #[test]
    fn rarity_increases_through_the_ladder() {
        let qualities = [
            Quality::Common,
            Quality::Uncommon,
            Quality::Rare,
            Quality::Epic,
            Quality::Legendary,
            Quality::Mythic,
            Quality::Ancient,
            Quality::Relic,
            Quality::Runebound,
            Quality::Cursed,
            Quality::Eldritch,
            Quality::Forgotten,
            Quality::Fatebound,
            Quality::HelForged,
            Quality::Jotunnforged,
            Quality::Voidborn,
            Quality::Einherjar,
            Quality::AesirTouched,
            Quality::Worldforged,
            Quality::NornTouched,
            Quality::Primordial,
        ];

        for pair in qualities.windows(2) {
            assert!(pair[0].base_rarity() < pair[1].base_rarity());
        }
    }

    #[test]
    fn primordial_is_one_in_two_million_base_rarity() {
        assert_eq!(Quality::Primordial.base_rarity(), 2_000_000);
    }

    #[test]
    fn display_names_preserve_norse_spelling() {
        assert_eq!(Quality::Jotunnforged.name(), "Jötunnforged");
        assert_eq!(Quality::AesirTouched.name(), "Æsir-Touched");
    }
}
