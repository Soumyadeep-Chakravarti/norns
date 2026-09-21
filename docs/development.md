# Development

## Requirements

The preferred development environment is provided through Nix. The repository
uses a flake under `.nix/`.

Direnv can enter the environment automatically:

```bash
direnv allow
```

Or enter it directly:

```bash
nix develop ./.nix
```

## Workspace

```text
crates/
|-- norns-core
|-- norns-protocol
|-- norns-client
`-- norns-server
```

**Locked design:** A simulation crate may be introduced later for large-scale
progression and economy balancing.

## Quality Gate

Before committing changes, run:

```bash
cargo fmt --all
cargo clippy --workspace
cargo test --workspace
```

Changes should keep the workspace warning-free.

The workspace enables:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
```

Individual crates inherit this lint configuration.

## Core Development Rules

### Keep Core Pure

`norns-core` contains deterministic domain rules. Do not introduce database,
networking, terminal rendering, asynchronous runtime, or operating-system
dependencies unless the architecture is deliberately changed.

### Prefer Integer Arithmetic

Progression, currency, quality, and economy systems should use integer
representations where practical. Examples include `u64` experience, `u64`
gold, basis points for multipliers, and integer quality-roll ranges.

### Saturate Unbounded Progression

Systems intended to progress indefinitely should not silently wrap on integer
safer than wrapping.

### Server Authority

Anything affecting multiplayer progression or the shared economy must
eventually be validated by the server. The client communicates player intent;
it does not authoritatively determine rewards.

### Responsive Terminal UI

The client must not target one terminal resolution. Layouts should use Ratatui
constraints and adapt to the available area.

## Balance Constants

Do not assume early numeric values are final. Current balance placeholders
include gathering XP, gathering cycle times, specialization luck coefficients,
forge costs, and content level requirements.

A future simulation environment can evaluate progression rates, resource
are treated as stable.
## Strict Checks

The workspace treats compiler warnings, missing documentation, broken rustdoc
links, all Clippy lints, and Clippy's pedantic lints as errors. Run the full
local gate before submitting changes:

```bash
cargo fmt --all -- --check
cargo rustdoc -p norns-core --lib -- -D missing_docs -D rustdoc::broken_intra_doc_links -D rustdoc::bare_urls
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace --all-targets --all-features --locked
```

The same checks run in `.github/workflows/strict.yml`.
