# create-tailyew-app
# ✨ TailYew
![Docs.rs](https://img.shields.io/docsrs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)
![CI](https://github.com/apjames93/tailyew/actions/workflows/create-tailyew-app.yaml/badge.svg)
![WASM Ready](https://img.shields.io/badge/WASM-ready-blueviolet)
![Powered by Yew](https://img.shields.io/badge/Yew-0.21-blue)
![Components](https://img.shields.io/badge/components-50%2B-blue)
[![Demo](https://img.shields.io/badge/live-demo-0C66F0?logo=vercel&logoColor=white)](https://tailyew.com)

**A zero-config CLI to scaffold and bootstrap a production-ready [TailYew](https://tailyew.com) frontend SPA.**

> 🚀 Instantly spin up a modern Yew + TailwindCSS app!

Pure Rust & WASM in production — no JS needed to run your UI.

Note: Tailwind’s CLI (via npm) is still required for development builds, and the dev server runs on Trunk.

Dreaming of a totally Rust-native toolchain? Us too! We’re open to ideas and contributions.

---

## ✨ What is this?

`create-tailyew-app` is the official scaffolding tool for the [TailYew](https://tailyew.com) component system. It sets up a best-practices Yew + Tailwind starter so you can focus on features, not configuration.

You get:

- A ready-to-go Yew/WASM frontend with TailwindCSS (JIT mode)
- Live-reload via Trunk
- Dark mode + accessibility baked in
- Examples of atomic, composable UI via TailYew
- Smart Makefile for build, lint, and dev workflows
- **Small bundle sizes** and no JS dependencies (besides Tailwind tooling)
- [See live TailYew demos »](https://tailyew.com/demo/getting_started)

---

## 🛠️ Installation

```bash
cargo install create-tailyew-app
````

> Requires Rust 1.65+ and `npm` in your PATH.

---

## 🚀 Quick Start

```bash
create-tailyew-app <NAME> [--dest <BASE_DIR>]
```

* **`NAME`** — Your new project folder (required)
* **`--dest`** — (Optional) Output directory (default: `.`)

### Example

```bash
rustup target add wasm32-unknown-unknown   # if not already installed
cargo install trunk                        # if not already installed

create-tailyew-app tailyew-app
cd tailyew-app
npm install                                # for tailwindcss cli
make run-frontend
```

This launches your dev server at [http://localhost:9001](http://localhost:9001).

---

## 📁 What’s Generated

```text
<BASE_DIR>/<NAME>/
├─ frontend/
│  ├─ Cargo.toml           # Yew/WASM crate
│  ├─ tailwind.config.js   # TailwindCSS setup (pre-configured for TailYew)
│  ├─ src/                 # Rust app sources
│  ├─ static/              # HTML, safelist, assets, WASM pkg
│  ├─ Trunk.toml           # Trunk config (dev server/build)
│  ├─ index.html           # Trunk entry point
│  └─ Makefile             # Frontend build & serve tasks (Trunk + Tailwind)
├─ Makefile                # Root orchestrator (run-frontend, lint, format, etc)
└─ README.md               # This file
```

* **Tailwind Safelist**: `static/tailyew-safelist.html` is included for correct dark-mode & dynamic class extraction.
  * Refresh it after upgrading TailYew: from the project root run `make fe-copy-tailyew-safelist` (or inside `frontend/` run `make copy-tailyew-safelist`).
* **Live Reload**: Hot-reloading for rapid dev (edit, save, reload).

---

## ⚡ Powered by TailYew

* 🌗 **Dark mode** ready with Tailwind’s `dark:` utilities
* 🧹 **50+ components**: Buttons, Inputs, Tables, Modals, Charts, Accordions, and more
* 📙 **Markdown** auto-renders to beautiful, theme-aware TailYew elements
* 📊 **Charts** (Bar, Line, Bubble, Pie, Scatter) — *no JS required*
* 📝 **Composable Forms** — with validation and accessibility
* 🦠 **Pure Rust** — no JS/TS needed for your UI
* 📦 **Tiny WASM builds** — thanks to Tailwind JIT and no-runtime bloat
* 📝 **A11y**: ARIA, labeling, and more

Explore live components: [tailyew.com/demo/getting\_started](https://tailyew.com/demo/getting_started)

---

## 🎬 Dev server

```bash
cd <BASE_DIR>/<NAME>
make run-frontend
```

* Watches Rust sources (`frontend/src`, `frontend/static`)
* Rebuilds WASM + Tailwind via Trunk pre-build hook, auto-reloads browser
* Serves locally at [localhost:9001](http://localhost:9001)

---

## 🔧 Useful Makefile Targets

From your project root:

* **`make help` ** - in the generated project to see all available targets.
* **`make format`** — Format Rust sources
* **`make lint`** — Lint with Clippy
* **`make pretty`** — Format + Lint
* **`make run-frontend`** — Serve with hot-reload
* **`make fe-copy-tailyew-safelist`** — Refresh the TailYew safelist HTML from your local Cargo registry

## ❤️ Contributing

Help grow the Rust UI ecosystem:

1. Fork & clone this repo
2. Edit `starter/app/` to change the template
3. Tweak CLI in `src/`
4. Run `make pretty` to format & lint
5. Open a PR — all feedback welcome!

---

## 📚 Helpful Links

* [TailYew Docs & Demos](https://tailyew.com)
* [Open a Pull Request](https://github.com/apjames93/tailyew/compare)
* [Report a Bug](https://github.com/apjames93/tailyew/issues/new?template=bug_report.md)
* [Propose a Feature](https://github.com/apjames93/tailyew/issues/new?template=feature_request.md)

---

## 🔗 Related Projects

* 🦠 [Yew Framework](https://yew.rs/)
* 🎨 [TailwindCSS](https://tailwindcss.com/)
