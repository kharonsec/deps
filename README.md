# deps

Universal dependency analyzer. Works across Cargo, npm, and go mod.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
deps [COMMAND]
```

### Commands:
- `deps`: List all dependencies.
- `deps why <pkg>`: Show what depends on a package.
- `deps tree`: Show the full dependency tree.
- `deps outdated`: Show what can be updated (stubbed).

Example:
```bash
deps why clap
```
