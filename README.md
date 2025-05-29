![Crates.io](https://img.shields.io/crates/v/tailyew)
![Docs.rs](https://img.shields.io/docsrs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)

# 🌬️ TailYew  
> The Tailwind + Yew component system — built entirely in Rust.

TailYew is an open-source component system for [Yew](https://yew.rs) apps, built using idiomatic Rust and styled with Tailwind CSS. This repo includes the component crate, full demo/docs site, and the scaffolding CLI.

> 🎯 This README is for **contributors**. If you want to use TailYew in your project, check the [crate README](./crates/tailyew/README.md) or the [create-tailyew-app CLI](https://crates.io/crates/create-tailyew-app) to get started.

---

## 🛠️ Local Setup

This repo is a monorepo with:

- `crates/tailyew/` – the component crate (`tailyew`)
- `frontend/` – the live demo and docs site (Yew + WASM)
- `crates/create-tailyew-app/` – CLI for scaffolding new TailYew projects

You’ll need:

- Rust + WASM toolchain (`rustup`, `wasm32-unknown-unknown`, etc.)
- Node.js (for Tailwind CSS + dev server)
- `make` (used for all common workflows)

---

### ✅ Getting Started

Clone the repo and enter it:

```bash
git clone https://github.com/apjames93/tailyew.git
cd tailyew
````

Then:

```bash
# Install all Rust deps
cargo check

# Install required Node packages for the frontend
cd frontend
npm install
cd ..

# Run the full docs site with hot reload (frontend + component crate)
make run-frontend
```

You can now open [http://localhost:8080](http://localhost:8080) and see the live docs site running.

---

## 🧪 Common Dev Commands

```bash
make run-frontend     # Run frontend + crate (hot reload)
make pretty           # Format + lint everything
make watch-docs       # Watch + build Rust docs (crates/tailyew/docs.rs)
```

Need to run something just in the frontend?

```bash
make fe-build
make fe-run
```

Or just in the crate?

```bash
make tailyew-doc
make tailyew-test
```

---

## 🗂️ Folder Structure

```
.
├── crates/
│   ├── tailyew/               # Main component system crate
│   └── create-tailyew-app/   # CLI to scaffold new apps
├── frontend/                  # Demo/docs site (Yew + Tailwind)
├── docs/                      # GitHub Pages markdown (optional)
├── Makefile                   # Root orchestrator (delegates to subdirs)
```

---

## ✅ Contribution Checklist

Before submitting a PR:

* ✅ Run `make pretty` (formats + lints)
* ✅ Run `make release-check`
* ✅ Test both **light mode** and **dark mode**
* ✅ Update related demo pages in `frontend/src/pages/`
* ✅ Include **before/after screenshots** for visual changes

---

## 📂 Helpful Links

* 📥 [Open a Pull Request](https://github.com/apjames93/tailyew/compare)
* 🐛 [Report a Bug](https://github.com/apjames93/tailyew/issues/new?template=bug_report.md)
* 💡 [Propose a Feature](https://github.com/apjames93/tailyew/issues/new?template=feature_request.md)

---

## 🙌 Thank You

Whether you're submitting a PR, filing an issue, or just exploring the project — thank you for helping us grow the Rust UI ecosystem.

---

## 🔗 Related

* 🦀 [Yew Framework](https://yew.rs/)
* 🎨 [Tailwind CSS](https://tailwindcss.com/)

```