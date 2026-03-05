# CLAUDE.md

## Project Overview

Lumen Blocks is an ARIA-accessible, styled component library for Dioxus (Rust web framework), inspired by shadcn-ui and built on top of Dioxus Primitives. It uses Tailwind CSS v4 for styling with OKLCH color space theming.

## Development Commands

```bash
# Enter Nix development shell (provides all tools)
nix develop

# Or with direnv (auto-activates on cd)
direnv allow

# Serve the documentation site (component preview)
dx serve -p docsite --platform web

# Build the library
cargo build -p lumen-blocks

# Run tests
cargo test -p lumen-blocks

# Check all packages
cargo check --workspace

# Just commands
just dev-docsite     # Start docsite dev server
just build-docsite   # Build docsite for production
just pre-commit      # Format and build check
```

## Nix Flake

The project includes a Nix flake for reproducible development environments.

```bash
# Enter dev shell
nix develop

# Build the docsite
nix build .#docsite

# Build the library
nix build .#lumen-blocks
```

## Architecture

### Workspace Structure
- `blocks/` - Core component library (published as `lumen-blocks` crate)
- `docsite/` - Documentation website with component previews, porting from `~/projects/ui/apps/v4/app/` (shadcn-ui v4 reference app) to Dioxus

### Core Modules (blocks/src/)
- `components/` - Individual UI components (Button, Dialog, Tabs, etc.)
- `theme.rs` - OKLCH-based theming system with `ThemeProvider` and predefined themes
- `variants.rs` - CVA-style class variance authority system (`cva`, `cn`, `class_if`)
- `patterns.rs` - Controlled/uncontrolled component patterns (`ControlledState`)
- `focus_trap.rs` - Focus management for modals (`use_focus_trap`, `FocusTrap`)

### Component Patterns

**Controlled vs Uncontrolled**: Stateful components support both patterns:
- Uncontrolled: Component manages state internally, use `default_*` props
- Controlled: Parent manages state via `Signal<T>`, component calls callbacks

**Standard Props**: Components follow consistent patterns:
- `id: Option<String>` - Optional element ID
- `class: Option<String>` - Additional CSS classes
- `disabled: bool` - Disabled state
- `on_*_change` callbacks for state changes
- `#[props(extends = GlobalAttributes)]` for HTML attribute passthrough

**Data Attributes**: Components use shadcn-style data attributes:
- `data-slot="component-name"` - Styling hooks for each component part
- `data-state="open|closed|checked|unchecked|active|inactive"` - State-based styling

### Styling System

Uses Tailwind CSS v4 with CSS custom properties for theming:
- CSS variables defined in `docsite/tailwind.css` (e.g., `--primary`, `--background`)
- Custom data attribute variants for state styling (`data-open:`, `data-checked:`, etc.)
- Theme colors use OKLCH color space for perceptual uniformity

### Variant System (CVA-style)

```rust
use lumen_blocks::variants::{cva, cn};

let button = cva(
    "base-classes",
    &[("variant", &[("primary", "bg-primary"), ("secondary", "bg-secondary")])],
    &[("variant", "primary")],  // defaults
    &[],  // compound variants
);
let classes = button.apply(&[("variant", "secondary")]);
```

### Dependencies
- Dioxus 0.7 - Rust UI framework
- dioxus-primitives - Unstyled accessible component primitives
- lucide-dioxus - Icon library
- Tailwind CSS v4 - Styling (processed by `dx serve`)
