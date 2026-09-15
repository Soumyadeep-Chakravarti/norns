pub type Experience = u64;
pub type SkillLevel = u32;

const QUADRATIC_SCALE: Experience = 100;
const CUBIC_SCALE: Experience = 5;

#[must_use]
pub fn xp_for_level(level: SkillLevel) -> Experience {
    if level <= 1 {
        return 0;
    }

    let x = Experience::from(level - 1);

    let x2 = x.saturating_mul(x);
    let x3 = x2.saturating_mul(x);

    QUADRATIC_SCALE
        .saturating_mul(x2)
        .saturating_add(CUBIC_SCALE.saturating_mul(x3))
}

#[must_use]
pub fn level_from_xp(xp: Experience) -> SkillLevel {
    let mut low: SkillLevel = 1;
    let mut high: SkillLevel = 2;

    while xp_for_level(high) <= xp && high < SkillLevel::MAX {
        low = high;

        let next = high.saturating_mul(2);

        if next == high {
            break;
        }

        high = next;
    }

    while low + 1 < high {
        let mid = low + (high - low) / 2;

        if xp_for_level(mid) <= xp {
            low = mid;
        } else {
            high = mid;
        }
    }

    low
}

#[cfg(test)]
mod tests {
    use super::{level_from_xp, xp_for_level};

    #[test]
    fn level_one_requires_no_xp() {
        assert_eq!(xp_for_level(1), 0);
    }

    #[test]
    fn thresholds_map_to_their_levels() {
        for level in 1..=1_000 {
            assert_eq!(level_from_xp(xp_for_level(level)), level);
        }
    }

    #[test]
    fn experience_before_threshold_maps_to_previous_level() {
        for level in 2..=1_000 {
            assert_eq!(level_from_xp(xp_for_level(level) - 1), level - 1);
        }
    }
}
