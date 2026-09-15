#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Quality {
    Standard,
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
    pub const fn tier(self) -> u32 {
        match self {
            Self::Standard => 0,
            Self::Common => 1,
            Self::Uncommon => 2,
            Self::Rare => 3,
            Self::Epic => 4,
            Self::Legendary => 5,
            Self::Mythic => 6,
            Self::Ancient => 7,
            Self::Relic => 8,
            Self::Runebound => 9,
            Self::Cursed => 10,
            Self::Eldritch => 11,
            Self::Forgotten => 12,
            Self::Fatebound => 13,
            Self::HelForged => 14,
            Self::Jotunnforged => 15,
            Self::Voidborn => 16,
            Self::Einherjar => 17,
            Self::AesirTouched => 18,
            Self::Worldforged => 19,
            Self::NornTouched => 20,
            Self::Primordial => 21,
        }
    }

    #[must_use]
    pub const fn base_rarity(self) -> u64 {
        match self {
            Self::Standard => 1,
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
    pub const fn next(self) -> Option<Self> {
        match self {
            Self::Standard => Some(Self::Common),
            Self::Common => Some(Self::Uncommon),
            Self::Uncommon => Some(Self::Rare),
            Self::Rare => Some(Self::Epic),
            Self::Epic => Some(Self::Legendary),
            Self::Legendary => Some(Self::Mythic),
            Self::Mythic => Some(Self::Ancient),
            Self::Ancient => Some(Self::Relic),
            Self::Relic => Some(Self::Runebound),
            Self::Runebound => Some(Self::Cursed),
            Self::Cursed => Some(Self::Eldritch),
            Self::Eldritch => Some(Self::Forgotten),
            Self::Forgotten => Some(Self::Fatebound),
            Self::Fatebound => Some(Self::HelForged),
            Self::HelForged => Some(Self::Jotunnforged),
            Self::Jotunnforged => Some(Self::Voidborn),
            Self::Voidborn => Some(Self::Einherjar),
            Self::Einherjar => Some(Self::AesirTouched),
            Self::AesirTouched => Some(Self::Worldforged),
            Self::Worldforged => Some(Self::NornTouched),
            Self::NornTouched => Some(Self::Primordial),
            Self::Primordial => None,
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
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

    const QUALITY_LADDER: [Quality; 22] = [
        Quality::Standard,
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

    #[test]
    fn standard_is_guaranteed() {
        assert_eq!(Quality::Standard.base_rarity(), 1);
    }

    #[test]
    fn rarity_increases_through_the_ladder() {
        for pair in QUALITY_LADDER.windows(2) {
            assert!(pair[0].base_rarity() < pair[1].base_rarity());
        }
    }

    #[test]
    fn tiers_increase_through_the_ladder() {
        for (tier, quality) in QUALITY_LADDER.iter().enumerate() {
            assert_eq!(quality.tier(), tier as u32);
        }
    }

    #[test]
    fn every_quality_except_primordial_has_a_next_tier() {
        for quality in QUALITY_LADDER {
            if quality == Quality::Primordial {
                assert_eq!(quality.next(), None);
            } else {
                let next = quality.next().expect("quality should have a next tier");

                assert_eq!(next.tier(), quality.tier() + 1);
            }
        }
    }

    #[test]
    fn primordial_is_the_final_quality() {
        assert_eq!(Quality::Primordial.next(), None);
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
