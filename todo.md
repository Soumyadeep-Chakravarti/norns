# Norns TODO

## 18-Tier Material Progression

- [ ] Replace individual ore and ingot identities with `MaterialTier`-based types.
- [ ] Add mining nodes and smelting recipes for all 18 material tiers.
- [ ] Add sword and axe recipes for all 18 tiers with quality resolution.
- [ ] Add equipment metadata for slots, combat families, and tier scaling.
- [ ] Add tier progression and quality/Forge coverage tests.
- [ ] Run formatting, Clippy, workspace tests, and review the final diff.

## Material Tiers

1. Copper
2. Tin
3. Bronze
4. Iron
5. Steel
6. Silver
7. Blackmetal
8. Obsidian
9. Root
10. Fenris
11. Carapace
12. Eitr
13. Dvergr
14. Jotunn
15. Aesir
16. Bifrost
17. Yggdrasil
18. Norn

Each equipment identity supports all 22 quality tiers, for 396 possible
material-quality combinations per identity.

## Items and Equipment

- [ ] Add equipment slots for head, chest, legs, hands, feet, main hand, off hand, cape, and accessory.
- [ ] Add item display names and descriptions.
- [ ] Add item categories and subcategories.
- [ ] Add stack limits and stack merging rules.
- [ ] Add item binding rules.
- [ ] Add durability and repair rules.
- [ ] Add equipment requirements.
- [ ] Add equipment stat budgets by material tier.
- [ ] Add melee weapon families.
- [ ] Add ranged weapon families.
- [ ] Add Seidr weapon families.
- [ ] Add armor families and armor weights.
- [ ] Add shields and off-hand items.
- [ ] Add tools with gathering bonuses.
- [ ] Add mounts and movement bonuses.
- [ ] Add item salvaging.
- [ ] Add item dismantling.
- [ ] Add item crafting commissions.

## Inventory and Player State

- [ ] Add authoritative player inventory state.
- [ ] Add inventory capacity.
- [ ] Add bank and storage containers.
- [ ] Add equipment loadouts.
- [ ] Add item movement transactions.
- [ ] Add item split and merge transactions.
- [ ] Add item discard confirmation.
- [ ] Add currency balances.
- [ ] Add activity state and completion timestamps.
- [ ] Add offline progress calculation.
- [ ] Add activity cancellation rules.
- [ ] Add player account persistence.
- [ ] Add save migration versioning.
- [ ] Add transaction idempotency.
- [ ] Add audit logs for economy mutations.

## Combat

- [ ] Define combat attributes.
- [ ] Define attack speed.
- [ ] Define accuracy and evasion.
- [ ] Define critical hits.
- [ ] Define armor mitigation.
- [ ] Define elemental and status effects.
- [ ] Define melee abilities.
- [ ] Define ranged abilities.
- [ ] Define Seidr abilities.
- [ ] Define enemy statistics.
- [ ] Define enemy resistances.
- [ ] Define enemy behavior profiles.
- [ ] Add combat rounds or deterministic combat ticks.
- [ ] Add combat victory and defeat outcomes.
- [ ] Add combat healing.
- [ ] Add death and recovery rules.
- [ ] Add combat XP.
- [ ] Add combat specialization XP.
- [ ] Add combat loot tables.
- [ ] Add boss phases.
- [ ] Add boss enrage mechanics.
- [ ] Add cooperative combat parties.
- [ ] Add world boss participation.
- [ ] Add combat simulation tests.

## World and Zones

- [ ] Add zone definitions.
- [ ] Add zone unlock requirements.
- [ ] Add zone entry costs.
- [ ] Add zone activity tables.
- [ ] Add zone gathering tables.
- [ ] Add zone monster tables.
- [ ] Add zone boss tables.
- [ ] Add zone drop modifiers.
- [ ] Add zone danger modifiers.
- [ ] Add Verdant Meadows content.
- [ ] Add Darkwood Forest content.
- [ ] Add Ancient Dungeon content.
- [ ] Add Shadowlands content.
- [ ] Add Molten Peak content.
- [ ] Add Frozen Wastes content.
- [ ] Add Abyssal Depths content.
- [ ] Add Drowned Ruins content.
- [ ] Add Stormcaller content.
- [ ] Add Celestial Realm content.
- [ ] Add chapter two and post-Celestial content.
- [ ] Add zone lore entries.
- [ ] Add unlock and travel UI.

## Gathering

- [ ] Add all 18 mining nodes.
- [ ] Add mining tool requirements.
- [ ] Add Woodcutting nodes.
- [ ] Add wood tiers and refined planks.
- [ ] Add Fishing spots.
- [ ] Add fish families and catches.
- [ ] Add Foraging locations.
- [ ] Add herbs and plants.
- [ ] Add Hunting locations.
- [ ] Add hides, meat, bones, and trophies.
- [ ] Add gathering depletion and respawn rules.
- [ ] Add gathering rare events.
- [ ] Add gathering resource quality-free guarantees.
- [ ] Add gathering specialization trees.
- [ ] Add gathering activity history.

## Production

- [ ] Add reusable recipe definitions.
- [ ] Add recipe ingredient validation.
- [ ] Add recipe output validation.
- [ ] Add production duration.
- [ ] Add production batch sizes.
- [ ] Add production level requirements.
- [ ] Add production specialization XP.
- [ ] Add Smithing armor recipes.
- [ ] Add Smithing tool recipes.
- [ ] Add Woodworking recipes.
- [ ] Add Leatherworking recipes.
- [ ] Add Cooking recipes.
- [ ] Add food effects and durations.
- [ ] Add Alchemy recipes.
- [ ] Add potion effects and durations.
- [ ] Add Runecrafting recipes.
- [ ] Add rune effects.
- [ ] Add glyph recipes.
- [ ] Add recipe discovery.
- [ ] Add recipe unlocks.
- [ ] Add production stations.
- [ ] Add station upgrades.
- [ ] Add production queues.

## Skills and Progression

- [ ] Add Divinity skill.
- [ ] Add Thieving skill.
- [ ] Add skill activity registries.
- [ ] Add activity level requirements.
- [ ] Add activity XP tables.
- [ ] Add skill milestone rewards.
- [ ] Add skill mastery rewards.
- [ ] Add specialization trees for every production skill.
- [ ] Add specialization respec rules.
- [ ] Add prestige or mastery beyond level 100.
- [ ] Add achievement progression.
- [ ] Add daily and weekly challenges.
- [ ] Add account-wide progression.
- [ ] Add seasonal progression.
- [ ] Add progression comparison tools.
- [ ] Add XP and balance simulation tooling.

## Divinity

- [ ] Add deity definitions.
- [ ] Add offering activities.
- [ ] Add offering item requirements.
- [ ] Add blessing unlocks.
- [ ] Add blessing durations.
- [ ] Add blessing stacking rules.
- [ ] Add shrine locations.
- [ ] Add divine currencies.
- [ ] Add divine specialization.
- [ ] Add divine world events.

## Thieving

- [ ] Add pickpocket targets.
- [ ] Add detection chance.
- [ ] Add suspicion state.
- [ ] Add escape rules.
- [ ] Add guards and pursuit.
- [ ] Add stall activities.
- [ ] Add heists.
- [ ] Add vaults.
- [ ] Add lockpicking progression.
- [ ] Add disguise progression.
- [ ] Add Thieving loot tables.
- [ ] Add Thieving cooldowns.
- [ ] Add Thieving consequences.

## Economy and Multiplayer

- [ ] Add player-to-player trading.
- [ ] Add trade confirmation.
- [ ] Add trade expiration.
- [ ] Add player-driven marketplace.
- [ ] Add buy orders.
- [ ] Add sell orders.
- [ ] Add market taxes.
- [ ] Add local market prices.
- [ ] Add price history.
- [ ] Add gold sinks.
- [ ] Add item sinks.
- [ ] Add clans.
- [ ] Add clan roles.
- [ ] Add clan storage.
- [ ] Add clan progression.
- [ ] Add cooperative expeditions.
- [ ] Add shared world events.
- [ ] Add gifting.
- [ ] Add crafting commissions.
- [ ] Add player reputation.
- [ ] Add chat channels.
- [ ] Add friends and ignore lists.
- [ ] Add presence and activity status.

## Server and Protocol

- [ ] Define player login flow.
- [ ] Define session authentication.
- [ ] Define authoritative command handling.
- [ ] Add activity start messages.
- [ ] Add activity completion messages.
- [ ] Add inventory synchronization.
- [ ] Add equipment synchronization.
- [ ] Add combat synchronization.
- [ ] Add market synchronization.
- [ ] Add chat synchronization.
- [ ] Add reconnect handling.
- [ ] Add optimistic UI reconciliation.
- [ ] Add rate limits.
- [ ] Add server-side validation errors.
- [ ] Add structured server logging.
- [ ] Add metrics and health checks.
- [ ] Add database migrations.
- [ ] Add backups and restore tests.
- [ ] Add admin moderation tools.

## Client Experience

- [ ] Add main game dashboard.
- [ ] Add skills screen.
- [ ] Add specialization screen.
- [ ] Add inventory screen.
- [ ] Add equipment screen.
- [ ] Add crafting screen.
- [ ] Add activity selection screen.
- [ ] Add activity progress screen.
- [ ] Add combat screen.
- [ ] Add zone screen.
- [ ] Add item inspection screen.
- [ ] Add quality display.
- [ ] Add Forge screen.
- [ ] Add marketplace screen.
- [ ] Add trade screen.
- [ ] Add clan screen.
- [ ] Add chat screen.
- [ ] Add notifications.
- [ ] Add keyboard shortcuts.
- [ ] Add accessibility options.
- [ ] Add colorblind-friendly palette.
- [ ] Add compact terminal layout.
- [ ] Add responsive terminal layout.
- [ ] Add client error recovery.

## Live Operations

- [ ] Add event definitions.
- [ ] Add limited-time activities.
- [ ] Add seasonal bosses.
- [ ] Add seasonal reward tracks.
- [ ] Add holiday content.
- [ ] Add rotating world modifiers.
- [ ] Add server announcements.
- [ ] Add patch notes display.
- [ ] Add maintenance mode.
- [ ] Add feature flags.
- [ ] Add content versioning.
- [ ] Add economy monitoring.
- [ ] Add exploit detection.
- [ ] Add rollback tooling.

## Testing and Quality

- [ ] Add unit tests for every item category.
- [ ] Add recipe property tests.
- [ ] Add quality distribution simulations.
- [ ] Add Forge economy simulations.
- [ ] Add XP curve simulations.
- [ ] Add combat balance simulations.
- [ ] Add offline progression tests.
- [ ] Add inventory transaction tests.
- [ ] Add persistence round-trip tests.
- [ ] Add protocol compatibility tests.
- [ ] Add server integration tests.
- [ ] Add client snapshot tests.
- [ ] Add load tests.
- [ ] Add fuzz tests for protocol input.
- [ ] Add fuzz tests for item transactions.
- [ ] Add deterministic replay tests.
- [ ] Add CI coverage reporting.
- [ ] Add release smoke tests.
- [ ] Add documentation consistency checks.

## Quests and Narrative

- [ ] Add quest definitions.
- [ ] Add quest prerequisites.
- [ ] Add quest objectives.
- [ ] Add multi-step quest chains.
- [ ] Add repeatable daily quests.
- [ ] Add repeatable weekly quests.
- [ ] Add zone introduction quests.
- [ ] Add boss unlock quests.
- [ ] Add gathering quests.
- [ ] Add crafting quests.
- [ ] Add combat quests.
- [ ] Add Divinity quest chains.
- [ ] Add Thieving quest chains.
- [ ] Add faction quest chains.
- [ ] Add quest rewards.
- [ ] Add choice-based quest outcomes.
- [ ] Add quest reputation.
- [ ] Add lore journal entries.
- [ ] Add NPC dialogue.
- [ ] Add NPC relationship levels.
- [ ] Add hidden lore discoveries.
- [ ] Add chapter completion rewards.

## Expeditions and Dungeons

- [ ] Add expedition definitions.
- [ ] Add expedition parties.
- [ ] Add expedition roles.
- [ ] Add expedition entry requirements.
- [ ] Add expedition preparation.
- [ ] Add expedition paths.
- [ ] Add branching encounters.
- [ ] Add expedition hazards.
- [ ] Add expedition checkpoints.
- [ ] Add expedition bosses.
- [ ] Add expedition completion grades.
- [ ] Add expedition time records.
- [ ] Add expedition reward chests.
- [ ] Add daily expedition modifiers.
- [ ] Add weekly expedition rotations.
- [ ] Add solo expeditions.
- [ ] Add group expeditions.
- [ ] Add endless dungeon floors.
- [ ] Add dungeon keys.
- [ ] Add dungeon relics.
- [ ] Add dungeon escape outcomes.

## World Events

- [ ] Add world event definitions.
- [ ] Add event start conditions.
- [ ] Add event participation tracking.
- [ ] Add event contribution scoring.
- [ ] Add cooperative event objectives.
- [ ] Add regional invasions.
- [ ] Add resource booms.
- [ ] Add monster migrations.
- [ ] Add corrupted resource nodes.
- [ ] Add traveling merchants.
- [ ] Add caravan escort events.
- [ ] Add lost expedition events.
- [ ] Add weather events.
- [ ] Add eclipse events.
- [ ] Add aurora events.
- [ ] Add world boss alerts.
- [ ] Add event reward milestones.
- [ ] Add event leaderboards without competitive combat.
- [ ] Add server-wide event completion rewards.

## Factions and Reputation

- [ ] Add faction definitions.
- [ ] Add faction reputation.
- [ ] Add faction ranks.
- [ ] Add faction contracts.
- [ ] Add faction vendors.
- [ ] Add faction currencies.
- [ ] Add faction-exclusive recipes.
- [ ] Add faction-exclusive equipment.
- [ ] Add faction-exclusive cosmetics.
- [ ] Add faction diplomacy.
- [ ] Add faction reputation decay rules.
- [ ] Add faction neutrality options.
- [ ] Add faction storylines.
- [ ] Add faction world events.
- [ ] Add faction contribution rewards.

## Companions and Creatures

- [ ] Add companion definitions.
- [ ] Add companion unlocks.
- [ ] Add companion rarity.
- [ ] Add companion levels.
- [ ] Add companion abilities.
- [ ] Add companion specialization.
- [ ] Add companion equipment.
- [ ] Add companion feeding.
- [ ] Add companion bonding.
- [ ] Add companion morale.
- [ ] Add companion collection bonuses.
- [ ] Add combat companions.
- [ ] Add gathering companions.
- [ ] Add crafting companions.
- [ ] Add cosmetic pets.
- [ ] Add mounts.
- [ ] Add mount speed tiers.
- [ ] Add mount stamina.
- [ ] Add mount breeding.
- [ ] Add mount cosmetic customization.

## Housing and Personal Progression

- [ ] Add player homesteads.
- [ ] Add housing locations.
- [ ] Add room upgrades.
- [ ] Add furniture items.
- [ ] Add furniture bonuses.
- [ ] Add crafting stations at home.
- [ ] Add storage upgrades.
- [ ] Add display cases.
- [ ] Add trophy displays.
- [ ] Add gardens.
- [ ] Add crop growth.
- [ ] Add livestock.
- [ ] Add worker assignments.
- [ ] Add home visitors.
- [ ] Add clan halls.
- [ ] Add clan hall upgrades.
- [ ] Add home decoration presets.
- [ ] Add housing achievements.

## Idle and Automation Systems

- [ ] Add offline activity summaries.
- [ ] Add offline reward previews.
- [ ] Add activity scheduling.
- [ ] Add activity priority queues.
- [ ] Add automatic resource collection.
- [ ] Add automatic production queues.
- [ ] Add automatic combat expeditions.
- [ ] Add automation unlock requirements.
- [ ] Add automation fuel or upkeep.
- [ ] Add worker assignments.
- [ ] Add retainer characters.
- [ ] Add retainer skill levels.
- [ ] Add expedition dispatches.
- [ ] Add return-time estimates.
- [ ] Add idle event discoveries.
- [ ] Add idle streak rewards.
- [ ] Add comeback rewards.
- [ ] Add catch-up mechanics for inactive players.
- [ ] Add activity efficiency reports.
- [ ] Add historical production graphs.

## Achievements and Collections

- [ ] Add achievement definitions.
- [ ] Add achievement categories.
- [ ] Add hidden achievements.
- [ ] Add progressive achievements.
- [ ] Add achievement points.
- [ ] Add achievement titles.
- [ ] Add achievement cosmetics.
- [ ] Add collection pages.
- [ ] Add item discovery tracking.
- [ ] Add monster discovery tracking.
- [ ] Add zone discovery tracking.
- [ ] Add recipe discovery tracking.
- [ ] Add lore discovery tracking.
- [ ] Add rare drop announcements.
- [ ] Add completion rewards.
- [ ] Add account completion percentage.
- [ ] Add clan achievement objectives.

## Cosmetics and Personal Identity

- [ ] Add cosmetic equipment overrides.
- [ ] Add weapon appearance overrides.
- [ ] Add armor appearance overrides.
- [ ] Add title selection.
- [ ] Add profile banners.
- [ ] Add profile emblems.
- [ ] Add chat titles.
- [ ] Add emotes.
- [ ] Add camp decorations.
- [ ] Add mount appearances.
- [ ] Add companion appearances.
- [ ] Add dye colors.
- [ ] Add cosmetic collections.
- [ ] Add cosmetic unlock achievements.
- [ ] Add cosmetic preview mode.

## Seasons and Long-Term Goals

- [ ] Add seasonal themes.
- [ ] Add seasonal activity tracks.
- [ ] Add seasonal quests.
- [ ] Add seasonal currencies.
- [ ] Add seasonal vendors.
- [ ] Add seasonal cosmetics.
- [ ] Add seasonal bosses.
- [ ] Add seasonal leaderboards.
- [ ] Add non-destructive leaderboard rewards.
- [ ] Add season-end summaries.
- [ ] Add legacy season achievements.
- [ ] Add mastery milestones.
- [ ] Add prestige cosmetics.
- [ ] Add long-term collection goals.
- [ ] Add account legacy bonuses.
