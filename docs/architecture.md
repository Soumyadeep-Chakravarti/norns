# Architecture

## Overview

**Locked design:** Norns separates deterministic game rules from presentation,
networking, persistence, and server infrastructure.

```text
                    +------------------+
                    |   norns-client   |
                    | Ratatui / Input  |
                    +--------+---------+
                             |
                      norns-protocol
                             |
                    +--------v---------+
                    |   norns-server   |
                    | authoritative    |
                    +--------+---------+
                             |
              +--------------+--------------+
              |                             |
       +------v-------+              +------v--------+
       | norns-core   |              | PostgreSQL     |
       | game rules   |              | persistence    |
       +--------------+              +---------------+
```

## Workspace

### `norns-core`

**Current implementation:** Contains pure game-domain logic, including
progression curves, skills, specialization, gathering rules, quality, forging,
and economy primitives.

It must not depend on Ratatui, Crossterm, Axum, SQLx, PostgreSQL, networking,
terminal dimensions, or server infrastructure. Game rules should be
deterministic whenever practical.

### `norns-protocol`

**Current implementation:** A shared workspace crate exists.

**Locked design:** It will contain types shared across the network boundary. It
will not contain authoritative game logic.

### `norns-client`

**Current implementation:** A client crate exists.

**Locked design:** It will provide Ratatui rendering, keyboard input,
navigation, responsive layouts, networking, and presentation state. The
interface should behave like a full-screen terminal application with panels,
keyboard navigation, live updates, and responsive layouts. It must not assume a
fixed terminal size.

### `norns-server`

**Current implementation:** A server crate exists.

**Locked design:** It will provide authentication, player sessions,
persistence, activity validation, offline progression, economy authority,
trading, multiplayer coordination, and server-generated randomness.

The client must never determine authoritative rewards.

## Deterministic Core

**Current implementation:** Quality rolling accepts a validated `QualityRoll`
input and deterministically resolves it to a quality.

**Locked design:** Randomness is generated outside `norns-core`.

```text
Server RNG
    |
    v
QualityRoll
    |
    v
norns-core
    |
    v
Quality
```

This keeps game rules reproducible and straightforward to test.

## Offline Progression

**Locked design:** Offline activity should be calculated from persisted activity
state rather than simulated continuously for every disconnected player.

```text
activity
target
started_at
cycle duration
progression state
```

On reconnect, the server calculates elapsed time, completed cycles, and
corresponding rewards. This keeps idle progression compatible with a
lightweight server.

## Authority Boundary

**Locked design:** The server is authoritative over shared progression and the
economy. The client presents state and submits player intent.

```text
Client: "I want to mine Copper."

Server: "Is this player allowed to mine Copper?"

Core: "Mining level satisfies the requirement."

Server: starts authoritative activity
```

The same principle applies to crafting, forging, trading, combat rewards, and
other economy-sensitive actions.
