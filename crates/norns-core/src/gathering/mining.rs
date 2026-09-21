use crate::{
    item::{ItemKind, ItemStack, MaterialTier, Resource},
    progression::{Experience, SkillLevel},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Configuration for one repeatable Mining activity.
pub struct MiningNode {
    resource: Resource,
    required_level: SkillLevel,
    cycle_seconds: u32,
    xp_per_cycle: Experience,
    specialization_xp_per_cycle: Experience,
}

impl MiningNode {
    #[must_use]
    /// Creates the provisional node configuration for a material tier.
    pub const fn for_tier(tier: MaterialTier) -> Self {
        let tier_number = tier.tier() as u32;

        Self {
            resource: Resource::TieredOre(tier),
            required_level: tier_number.saturating_mul(5),
            cycle_seconds: 3u32.saturating_add(tier_number.saturating_mul(2)),
            xp_per_cycle: (tier_number as u64).saturating_mul(10),
            specialization_xp_per_cycle: (tier_number as u64).saturating_mul(10),
        }
    }

    #[must_use]
    /// Returns the resource produced by the node.
    pub const fn resource(self) -> Resource {
        self.resource
    }

    #[must_use]
    /// Returns the required Mining level.
    pub const fn required_level(self) -> SkillLevel {
        self.required_level
    }

    #[must_use]
    /// Returns the provisional activity duration in seconds.
    pub const fn cycle_seconds(self) -> u32 {
        self.cycle_seconds
    }

    #[must_use]
    /// Returns broad Mining XP per cycle.
    pub const fn xp_per_cycle(self) -> Experience {
        self.xp_per_cycle
    }

    #[must_use]
    /// Returns resource specialization XP per cycle.
    pub const fn specialization_xp_per_cycle(self) -> Experience {
        self.specialization_xp_per_cycle
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Result of one completed Mining cycle.
pub struct MiningOutcome {
    stack: ItemStack,
    mining_xp: Experience,
    specialization_xp: Experience,
}

impl MiningOutcome {
    #[must_use]
    /// Returns the gathered resource stack.
    pub const fn stack(self) -> ItemStack {
        self.stack
    }

    #[must_use]
    /// Returns broad Mining XP.
    pub const fn mining_xp(self) -> Experience {
        self.mining_xp
    }

    #[must_use]
    /// Returns resource specialization XP.
    pub const fn specialization_xp(self) -> Experience {
        self.specialization_xp
    }
}

/// Provisional Stone node.
pub const STONE: MiningNode = MiningNode {
    resource: Resource::Stone,
    required_level: 1,
    cycle_seconds: 3,
    xp_per_cycle: 10,
    specialization_xp_per_cycle: 10,
};

/// Provisional Copper Ore node.
pub const COPPER: MiningNode = MiningNode {
    resource: Resource::CopperOre,
    required_level: 5,
    cycle_seconds: 5,
    xp_per_cycle: 20,
    specialization_xp_per_cycle: 20,
};

/// Provisional Tin Ore node.
pub const TIN: MiningNode = MiningNode {
    resource: Resource::TinOre,
    required_level: 10,
    cycle_seconds: 6,
    xp_per_cycle: 30,
    specialization_xp_per_cycle: 30,
};

/// Provisional Iron Ore node.
pub const IRON: MiningNode = MiningNode {
    resource: Resource::IronOre,
    required_level: 15,
    cycle_seconds: 8,
    xp_per_cycle: 40,
    specialization_xp_per_cycle: 40,
};

#[must_use]
/// Checks the Mining level requirement for a node.
pub const fn can_mine(node: MiningNode, mining_level: SkillLevel) -> bool {
    mining_level >= node.required_level
}

#[must_use]
/// Resolves one Mining cycle without mutating player state.
pub const fn mine(node: MiningNode) -> MiningOutcome {
    let Some(stack) = ItemStack::new(ItemKind::Resource(node.resource), 1) else {
        unreachable!()
    };

    MiningOutcome {
        stack,
        mining_xp: node.xp_per_cycle,
        specialization_xp: node.specialization_xp_per_cycle,
    }
}

#[cfg(test)]
mod tests {
    use crate::item::{ItemKind, MaterialTier, Resource};

    use super::{COPPER, IRON, MiningNode, STONE, TIN, can_mine, mine};

    #[test]
    fn level_requirement_is_enforced() {
        assert!(can_mine(STONE, 1));

        assert!(!can_mine(COPPER, 4));
        assert!(can_mine(COPPER, 5));

        assert!(!can_mine(TIN, 9));
        assert!(can_mine(TIN, 10));

        assert!(!can_mine(IRON, 14));
        assert!(can_mine(IRON, 15));
    }

    #[test]
    fn mining_produces_expected_outcome() {
        let outcome = mine(COPPER);
        let stack = outcome.stack();

        assert_eq!(stack.kind(), ItemKind::Resource(Resource::CopperOre));
        assert_eq!(stack.quantity(), 1);
        assert_eq!(outcome.mining_xp(), COPPER.xp_per_cycle());
        assert_eq!(
            outcome.specialization_xp(),
            COPPER.specialization_xp_per_cycle()
        );
    }

    #[test]
    fn every_material_tier_has_a_mining_node() {
        for tier in MaterialTier::ALL {
            let node = MiningNode::for_tier(tier);
            assert_eq!(node.resource(), Resource::TieredOre(tier));
            assert!(node.required_level() > 0);
            assert!(node.xp_per_cycle() > 0);
        }
    }
}
