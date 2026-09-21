mod smelting;
mod smithing;

pub use smelting::{
    COPPER_INGOT, IRON_INGOT, SmeltingOutcome, SmeltingRecipe, TIN_INGOT, can_smelt, smelt,
};
pub use smithing::{
    COPPER_AXE, COPPER_SWORD, IRON_AXE, IRON_SWORD, SmithingOutcome, SmithingRecipe, TIN_AXE,
    TIN_SWORD, can_smith, smith,
};
