//! Deterministic, persistence- and transport-independent rules for Norns.
//!
//! The crate intentionally separates three responsibilities:
//!
//! - domain values describe items, quality, skills, currency, and progression;
//! - activity functions calculate outcomes without owning player state;
//! - inventory transaction methods apply validated outcomes to owned stacks.
//!
//! The server is expected to own randomness, persistence, authorization, and
//! transaction orchestration. Core functions accept server-supplied quality
//! rolls so the same inputs always produce the same result.
#![deny(missing_docs)]

/// Deterministic combat definitions and round resolution.
pub mod combat;
/// Crafting recipes and production outcomes.
pub mod crafting;
/// Currency and economy primitives.
pub mod economy;
/// Gathering activities and resource outcomes.
pub mod gathering;
/// Item identity, quality, equipment, inventory, and Forge rules.
pub mod item;
/// Experience curves and specialization progression.
pub mod progression;
/// Skill taxonomy and broad skill progress.
pub mod skill;
