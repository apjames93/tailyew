# create-tailyew-app

**A zero-config starter template for building Rust frontends with [Yew](https://yew.rs/) + [Tailwind CSS](https://tailwindcss.com/), powered by the [TailYew](https://github.com/tailyew/tailyew) component system.**

[![MIT licensed](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/made_with-rust-orange.svg)](https://www.rust-lang.org/)
[![Yew](https://img.shields.io/badge/framework-yew-ff5757.svg)](https://yew.rs/)
[![TailwindCSS](https://img.shields.io/badge/styling-tailwindcss-38bdf8?logo=tailwindcss&logoColor=white)](https://tailwindcss.com/)

---

## ✨ What is this?

This is a scaffold app for TailYew, accessible, and fully typed UI component library built with Yew and styled using Tailwind. This project provides:

- 🔥 Instant setup with Yew 0.21 and Tailwind CSS
- 🧱 Example components and pages
- 🚀 Production-ready file structure
- 🧠 Designed around [Atomic Design](https://bradfrost.com/blog/post/atomic-web-design/)

---

## 📦 System Requirements

You'll need the following tools installed:

- [Rust](https://rust-lang.org/tools/install)
- `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- `npm install` (for Tailwind CLI)

## 🚀 Quickstart

```sh
# Clone the scaffold
git clone https://github.com/tailyew/create-tailyew-app
cd create-tailyew-app

cargo build
npm install

# Start the dev server with hot reloads (Trunk)
make run
```

Visit: [http://localhost:9001](http://localhost:9001)

---
---

## 🧩 Using TailYew Components

To use TailYew components in your Yew project:

1. Add the dependency:

```sh
cargo add tailyew
```

2. Import and use components:

```rust
use tailyew::atoms::Button;

html! {
    <Button label="Click me!" />
}
```

3. Tailwind classes are auto-applied via the included `tailwind.config.js` and `main.css`. Custom theme colors and fonts are already configured.

---

## 📁 Project Structure

```
├── src/
│   ├── pages/           # Landing + NotFound pages
│   ├── templates/       # Layout components (e.g., NavBar)
│   ├── lib.rs           # App entry point + routing
│   └── app_router.rs    # Main router with layout shell
├── static/             # assets copied by Trunk (manifest, service worker, images)
│   └── pkg/            # wasm build output (generated)
├── index.html          # Trunk entrypoint
├── Trunk.toml          # Trunk configuration
├── main.css             # Tailwind setup + custom themes
├── tailwind.config.js   # Design tokens and scan paths
├── Cargo.toml           # Rust + wasm config
└── Makefile             # Dev commands (build, run, lint)
```

---

## 📦 Built With

- [Yew](https://yew.rs/) — Rust-based web framework (WASM)
- [TailYew](https://github.com/tailyew/tailyew) — Reusable component system
- [Tailwind CSS](https://tailwindcss.com/) — Utility-first styling
- [Trunk](https://trunkrs.dev) — Build + serve

---

## 🤝 Contributing

We welcome contributions! Here's how you can get involved:

- Open issues for bugs or ideas
- Suggest new components or improvements
- Submit a PR (component demos, docs, DX tools, etc.)

This template is part of the [TailYew](https://github.com/apjames93/tailyew) ecosystem. Contributions to TailYew itself are also appreciated.

---

## 📄 License

- [MIT License](LICENSE)
