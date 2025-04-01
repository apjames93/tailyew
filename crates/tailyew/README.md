<!-- crates/tailyew/README.md -->
# 🧩 TailYew – Reusable Yew + Tailwind Component Library

TailYew is a **modular, reusable UI component system** for [Yew](https://yew.rs) built with the utility-first power of [Tailwind CSS](https://tailwindcss.com). It provides elegant, accessible, and composable components to build Rust web frontends faster and with consistency.

This crate is used internally in our own documentation site — see [`../frontend/`](../../frontend) — which showcases each component with live usage and examples.

---

## 🏗️ Project Goals

- 💡 **Atomic Design** pattern (atoms → molecules → organisms)
- ⚙️ **Yew-native** components — idiomatic Rust, no JavaScript
- 🎨 **Tailwind-first** styling — utility-based, flexible, consistent
- 🧪 **Well-documented** 
- 🧩 **Composable API** for building custom UIs

---

## 📁 Folder Structure

```bash
crates/tailyew/
├── src/
│   ├── atoms/         # Low-level building blocks (buttons, inputs, etc.)
│   ├── molecules/     # Mid-level UI (forms, modals, selects)
│   ├── organisms/     # Full UI blocks (headers, footers, nav)
│   ├── charts/        # Data visualization components (optional)
│   ├── form/          # Form layout & state handling helpers
│   ├── icons/         # Custom SVG icon components
│   └── lib.rs         # Exports all components and modules
├── Makefile           # Dev commands (build, docs)
├── Cargo.toml         # Rust crate manifest
└── README.md          # You are here
```

---

## 🧰 Development

This crate is part of a Cargo workspace. You can develop it in isolation or alongside the documentation frontend.

### 🔧 Build

```bash
make build
```

### 📚 Generate docs

```bash
make doc
```

Or skip opening the browser:

```bash
make doc-no-open
```

---

## 🧪 Testing in the Docs Site

To see components in use:

1. Run `make run-frontend` from the root
2. Edit components in this crate (`src/atoms/`, etc.)
3. Rebuild and preview live in `frontend/`

Hot reloading is handled via `cargo watch` from the root Makefile. The docs site imports this crate directly via:

```toml
# frontend/Cargo.toml
tailyew = { path = "../crates/tailyew" }
```

---

## 🚀 Publishing

Before publishing, ensure tests and docs pass:

```bash
make doc-no-open
```

Then:

```bash
make publish-tailyew
```

---

## 📎 Related

- [`frontend/`](../../frontend) – Docs site showcasing TailYew
- [`Makefile`](../../Makefile) – Root build & dev orchestrator
- [Yew Framework](https://yew.rs/)
- [Tailwind CSS](https://tailwindcss.com/)

---