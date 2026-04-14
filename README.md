# deps - Universal Dependency Analyzer

`deps` is a universal dependency analyzer that works across Cargo (Rust), npm (Node.js), and go mod (Go). It helps you list dependencies, visualize the dependency tree, and find out which packages depend on a specific dependency.

## Installation

### One-liner (requires Rust/Cargo)
```bash
curl -fsSL https://raw.githubusercontent.com/kharonsec/deps/master/install.sh | bash
```

### Manual
```bash
git clone https://github.com/kharonsec/deps.git
cd deps
./install.sh
```

## Usage

### List all dependencies
```bash
deps list
```

### Find what depends on a package
```bash
deps why <package_name>
```

### Show the dependency tree
```bash
deps tree
```

### Check for outdated dependencies (stub)
```bash
deps outdated
```
