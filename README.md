# dioxus-shadcn

A high-fidelity port of [shadcn UI](https://ui.shadcn.com) to [Dioxus](https://dioxuslabs.com), forked from [Lumen Blocks](https://github.com/lumenblocks/lumen).

## Features

- 40+ components with shadcn-ui patterns
- OKLCH color theming with multiple presets
- Full ARIA accessibility via Dioxus Primitives
- Tailwind CSS v4

## Development

```bash
nix develop                           # enter dev environment
dx serve -p docsite --platform web    # serve docsite
cargo check -p lumen-blocks           # check compilation
```

## License

[MIT](./LICENSE)
