# AGENTS.md

## Overview

This document defines the behavior, roles, and constraints of agents involved in the `rustybackup` project. While the current implementation is manual, the structure anticipates the integration of intelligent or automated agents (e.g., Codex, file monitoring daemons, or task-specific AI modules).

## Agent: `Codex`

**Type**: Interactive code generation assistant
**Role**: Planning, scaffolding, explaining, and implementing modules in Rust

### Permissions:

* ✅ Full read/write access to local project structure
* ❌ No uncontrolled filesystem access outside project root
* ❌ No persistent execution privileges
* 🟡 Optional internet access: only to the following domains:

  * `index.crates.io`
  * `crates.io`
  * `static.crates.io`
  * `github.com/rust-lang/crates.io-index`

### Responsibilities:

* Understand project intent and modular structure
* Suggest new functions, modules, and config patterns
* Respect project constraints (immutability, versioning safety)
* Avoid introducing unsafe or irreversible operations

This document will evolve as new agents are added to the system.
