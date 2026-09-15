use crate::{
    item::Resource,
    progression::{Experience, SkillLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MiningNode {
    resource: Resource,
    required_level: SkillLevel,
    cycle_seconds: u32,
    xp_per_cycle: Experience,
    specialization_xp_per_cycle: Experience,
}

impl MiningNode {
    #[must_use]
    pub const fn resource(self) -> Resource {
        self.resource
    }

    #[must_use]
    pub const fn required_level(self) -> SkillLevel {
        self.required_level
    }

    #[must_use]
    pub const fn cycle_seconds(self) -> u32 {
        self.cycle_seconds
    }

    #[must_use]
    pub const fn xp_per_cycle(self) -> Experience {
        self.xp_per_cycle
    }

    #[must_use]
    pub const fn specialization_xp_per_cycle(self) -> Experience {
        self.specialization_xp_per_cycle
    }
}

pub const STONE: MiningNode = MiningNode {
    resource: Resource::Stone,
    required_level: 1,
    cycle_seconds: 3,
    xp_per_cycle: 10,
    specialization_xp_per_cycle: 10,
};

pub const COPPER: MiningNode = MiningNode {
    resource: Resource::CopperOre,
    required_level: 5,
    cycle_seconds: 5,
    xp_per_cycle: 20,
    specialization_xp_per_cycle: 20,
};

pub const TIN: MiningNode = MiningNode {
    resource: Resource::TinOre,
    required_level: 10,
    cycle_seconds: 6,
    xp_per_cycle: 30,
    specialization_xp_per_cycle: 30,
};

#[must_use]
pub const fn can_mine(node: MiningNode, mining_level: SkillLevel) -> bool {
    mining_level >= node.required_level
}

#[cfg(test)]
mod tests {
    use super::{COPPER, STONE, TIN, can_mine};

    #[test]
    fn level_requirement_is_enforced() {
        assert!(can_mine(STONE, 1));

        assert!(!can_mine(COPPER, 4));
        assert!(can_mine(COPPER, 5));

        assert!(!can_mine(TIN, 9));
        assert!(can_mine(TIN, 10));
    }
}
