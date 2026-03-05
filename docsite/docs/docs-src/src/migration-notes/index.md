# Migration Notes

Version migration guide for dioxus-shadcn.

## v0.1 to v0.2

### Changes

1. Dioxus 0.7 (was 0.6)
2. Tailwind v4 (was v3)
3. Minor component interface changes

### Steps

1. Update Dioxus CLI to 0.7
2. Update dependencies to Dioxus 0.7
3. Update dioxus-shadcn dependency
4. Refactor `tailwind.css` for Tailwind v4 (see [Installation Guide](../installation/index.md))
5. Rename `tailwind.config.js` to `tailwind-config.js` and update format
6. Update [Dropdown](../dropdown/index.md) and [Menubar](../menubar/index.md) usages per documentation
