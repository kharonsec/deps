use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use petgraph::graph::NodeIndex;
use petgraph::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "deps")]
#[command(author, version, about = "Universal dependency analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all dependencies
    List,
    /// Show why a package is depended on
    Why { package: String },
    /// Show the dependency tree
    Tree,
    /// Check for outdated dependencies (stub)
    Outdated,
}

#[derive(Deserialize)]
struct CargoToml {
    dependencies: Option<HashMap<String, serde::Value>>,
}

#[derive(Deserialize)]
struct PackageJson {
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
}

struct Dependency {
    name: String,
    version: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = std::env::current_dir()?;

    match cli.command.unwrap_or(Commands::List) {
        Commands::List => {
            let deps = collect_dependencies(&root)?;
            for dep in deps {
                println!("{}: {}", dep.name, dep.version);
            }
        }
        Commands::Why { package } => {
            let tree = build_dependency_tree(&root)?;
            let nodes: Vec<_> = tree.node_indices().filter(|&i| tree[i] == package).collect();
            if nodes.is_empty() {
                println!("Package '{}' not found in dependencies.", package);
            } else {
                for node in nodes {
                    println!("Package '{}' is depended on by:", package);
                    let mut incoming = tree.neighbors_directed(node, Direction::Incoming);
                    while let Some(parent) = incoming.next() {
                        println!(" - {}", tree[parent]);
                    }
                }
            }
        }
        Commands::Tree => {
            let tree = build_dependency_tree(&root)?;
            print_tree(&tree, NodeIndex::new(0), 0);
        }
        Commands::Outdated => {
            println!("Outdated check is not implemented yet (requires network).");
        }
    }

    Ok(())
}

fn collect_dependencies(root: &Path) -> Result<Vec<Dependency>> {
    let mut deps = Vec::new();

    if root.join("Cargo.toml").exists() {
        let content = fs::read_to_string(root.join("Cargo.toml"))?;
        let cargo: CargoToml = toml::from_str(&content).context("Failed to parse Cargo.toml")?;
        if let Some(d) = cargo.dependencies {
            for (name, val) in d {
                let version = match val {
                    serde::Value::String(s) => s,
                    _ => "unknown".to_string(),
                };
                deps.push(Dependency { name, version });
            }
        }
    } else if root.join("package.json").exists() {
        let content = fs::read_to_string(root.join("package.json"))?;
        let pkg: PackageJson = serde_json::from_str(&content).context("Failed to parse package.json")?;
        if let Some(d) = pkg.dependencies {
            for (name, version) in d {
                deps.push(Dependency { name, version });
            }
        }
        if let Some(d) = pkg.dev_dependencies {
            for (name, version) in d {
                deps.push(Dependency { name, version });
            }
        }
    } else if root.join("go.mod").exists() {
        let content = fs::read_to_string(root.join("go.mod"))?;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("require") && !trimmed.contains("(") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 {
                    deps.push(Dependency { name: parts[1].to_string(), version: parts[2].to_string() });
                }
            }
        }
    }

    Ok(deps)
}

fn build_dependency_tree(root: &Path) -> Result<Graph<String, ()>> {
    let mut graph = Graph::<String, ()>::new();
    let root_node = graph.add_node("root".to_string());
    
    let deps = collect_dependencies(root)?;
    for dep in deps {
        let dep_node = graph.add_node(dep.name);
        graph.add_edge(root_node, dep_node, ());
    }
    
    Ok(graph)
}

fn print_tree(graph: &Graph<String, ()>, node: NodeIndex, indent: usize) {
    println!("{}{}", "  ".repeat(indent), graph[node]);
    let mut neighbors = graph.neighbors_directed(node, Direction::Outgoing);
    while let Some(neighbor) = neighbors.next() {
        print_tree(graph, neighbor, indent + 1);
    }
}
