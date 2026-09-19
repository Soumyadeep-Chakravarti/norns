# Norns

Norns is a multiplayer idle RPG built natively for the terminal.

It combines long-term skill progression, specialization, crafting, gathering,
cooperative multiplayer systems, and a player-driven economy in a responsive
full-screen terminal interface. The setting and visual identity are inspired
by Nordic mythology.

> Norns is in early development. Game balance, content, and protocol design
> are subject to change.

## Documentation Status

The documentation uses the following labels:

- **Locked design**: an intended project direction.
- **Current implementation**: present in the codebase today.
- **Balance placeholder**: a provisional number that requires balancing.

## Design Goals

- Native terminal interface
- Infinite long-term progression
- Deep per-item and per-resource specialization
- Meaningful player economy
- Cooperative multiplayer rather than PvP
- Lightweight authoritative server
- Deterministic and testable game rules
- Offline idle progression
- Portable native client

## Skills

```text
NORNS
|-- Gathering
|   |-- Mining
|   |-- Woodcutting
|   |-- Fishing
|   |-- Foraging
|   `-- Hunting
|-- Crafting
|   |-- Smithing
|   |-- Woodworking
|   |-- Cooking
|   |-- Leatherworking
|   |-- Alchemy
|   `-- Runecrafting
`-- Combat
    |-- Melee
    |-- Ranged
    `-- Seidr
```

Gathering extracts resources. Crafting transforms resources. Combat provides
access to threats, regions, materials, and encounters.

## Specialization

Broad skills and individual activities progress independently.

```text
Mining Lv. 143
|-- Stone Spec. 93
|-- Copper Spec. 72
|-- Tin Spec. 41
`-- Iron Spec. 118
```

Mining Iron grants both Mining experience and Iron specialization experience.
The main skill determines content access; specialization represents mastery of
a particular resource, item, weapon, recipe, or activity.

Both systems support progression beyond level 100.

## Quality

Quality-bearing item categories use a long-tail quality system ranging from
Standard to Primordial. Quality can be obtained through natural quality rolls
or deterministically through the Forge for eligible items.

Raw gathered resources have no quality. Their `ItemStack` contains only item
identity and a positive quantity; Mining produces these stacks alongside XP.
The concrete model for quality-bearing equipment and crafted items will be
defined when the first such category is implemented.

See [Quality and Forging](docs/quality-and-forging.md).

## Multiplayer

Norns is designed around cooperative and social multiplayer systems rather
than PvP. Planned systems include trading, player-driven markets, clans,
gifting, crafting commissions, world bosses, shared expeditions, world events,
and chat.

The server is authoritative over progression and the economy.

## Architecture

Norns is a Rust workspace:

```text
crates/
|-- norns-core
|-- norns-protocol
|-- norns-client
`-- norns-server
```

The core game rules are isolated from networking, persistence, terminal
rendering, and asynchronous runtime concerns.

See [Architecture](docs/architecture.md).

## Technology

- Rust
- Ratatui
- Crossterm
- Tokio
- Axum
- PostgreSQL
- SQLx
- Serde
- Cargo
- Nix
- GitHub Actions

## Development

Enter the development environment:

```bash
direnv allow
```

Or directly:

```bash
nix develop ./.nix
```

Run project checks:

```bash
cargo fmt --all
cargo clippy --workspace
cargo test --workspace
```

See [Development](docs/development.md).

## Status

Current implementation includes foundations for skill taxonomy, infinite XP
progression, specialization progression, mining, resources, item quality,
deterministic quality rolling, gold, and forging.

Balance constants in the codebase are placeholders until dedicated balancing
and simulation work begins.

## License

Norns is intended to be source-available rather than open source. A dedicated
license defining permitted use, modification, redistribution, hosting, and
commercial use will be added separately.
