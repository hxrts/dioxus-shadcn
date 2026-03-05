# Installation

Setup guide for dioxus-shadcn.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/getting_started/#installing-the-cli)

## New Project

Clone the starter template:
```bash
git clone https://github.com/hxrts/dioxus-shadcn-starter.git my-app
cd my-app
dx serve
```

## Existing Project

Add to your `Cargo.toml`:
```toml
dioxus-shadcn = { git = "https://github.com/hxrts/dioxus-shadcn.git", tag = "v0.3.0" }
```

Create `tailwind.css`:
```css
@import "tailwindcss";
@config "./tailwind-config.js";

body {
    background-color: var(--background);
    color: var(--foreground);
}

:root {
    --background: oklch(1 0 0);
    --foreground: oklch(0.145 0 0);
    --card: oklch(1 0 0);
    --card-foreground: oklch(0.145 0 0);
    --popover: oklch(1 0 0);
    --popover-foreground: oklch(0.145 0 0);
    --primary: oklch(0.205 0 0);
    --primary-foreground: oklch(0.985 0 0);
    --secondary: oklch(0.97 0 0);
    --secondary-foreground: oklch(0.205 0 0);
    --muted: oklch(0.97 0 0);
    --muted-foreground: oklch(0.556 0 0);
    --accent: oklch(0.97 0 0);
    --accent-foreground: oklch(0.205 0 0);
    --destructive: oklch(0.5757 0.2352 27.92);
    --destructive-foreground: oklch(0.577 0.245 27.325);
    --border: oklch(0.922 0 0);
    --input: oklch(0.922 0 0);
    --ring: oklch(0.708 0 0);
    --radius: 0.625rem;
}

@media (prefers-color-scheme: dark) {
    :root {
        --background: oklch(0.145 0 0);
        --foreground: oklch(0.985 0 0);
        --card: oklch(0.145 0 0);
        --card-foreground: oklch(0.985 0 0);
        --popover: oklch(0.145 0 0);
        --popover-foreground: oklch(0.985 0 0);
        --primary: oklch(0.985 0 0);
        --primary-foreground: oklch(0.205 0 0);
        --secondary: oklch(0.269 0 0);
        --secondary-foreground: oklch(0.985 0 0);
        --muted: oklch(0.269 0 0);
        --muted-foreground: oklch(0.708 0 0);
        --accent: oklch(0.269 0 0);
        --accent-foreground: oklch(0.985 0 0);
        --destructive: oklch(0.5058 0.2066 27.85);
        --border: oklch(0.269 0 0);
        --input: oklch(0.269 0 0);
        --ring: oklch(0.439 0 0);
    }
}

@theme inline {
    --color-background: var(--background);
    --color-foreground: var(--foreground);
    --color-card: var(--card);
    --color-card-foreground: var(--card-foreground);
    --color-popover: var(--popover);
    --color-popover-foreground: var(--popover-foreground);
    --color-primary: var(--primary);
    --color-primary-foreground: var(--primary-foreground);
    --color-secondary: var(--secondary);
    --color-secondary-foreground: var(--secondary-foreground);
    --color-muted: var(--muted);
    --color-muted-foreground: var(--muted-foreground);
    --color-accent: var(--accent);
    --color-accent-foreground: var(--accent-foreground);
    --color-destructive: var(--destructive);
    --color-destructive-foreground: var(--primary-foreground);
    --color-border: var(--border);
    --color-input: var(--input);
    --color-ring: var(--ring);
    --radius-sm: calc(var(--radius) - 4px);
    --radius-md: calc(var(--radius) - 2px);
    --radius-lg: var(--radius);
    --radius-xl: calc(var(--radius) + 4px);
}
```

Create `tailwind-config.js`:
```js
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    // Include dioxus-shadcn components
    `${process.env.HOME}/.cargo/git/checkouts/dioxus-shadcn-*/*/blocks/src/**/*.rs`
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

**Note:** The file must be named `tailwind-config.js` (not `tailwind.config.js`) for Dioxus CLI to use Tailwind v4.

**Windows:** Replace `${process.env.HOME}` with `${process.env.USERPROFILE}` or use an absolute path.

## Troubleshooting

**Tailwind classes not applied:**
- Check `tailwind-config.js` content paths
- Verify Dioxus CLI generates the output CSS
- Ensure you import the output `tailwind.css`

**Components not rendering:**
- Verify imports are correct
- Check version compatibility

[Open an issue](https://github.com/hxrts/dioxus-shadcn/issues) if problems persist.
