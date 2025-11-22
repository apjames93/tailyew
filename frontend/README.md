<!-- frontend/README.md -->

# 🌐 TailYew Frontend Docs

This is the frontend documentation site for the [`tailyew`](../crates/tailyew) component library — a modern UI system for [Yew](https://yew.rs/) built with [Tailwind CSS](https://tailwindcss.com/).

It’s a **real Yew application** that imports and displays live usage of TailYew components. This site acts as a visual playground, reference, and dogfooding environment for building and testing new components in context.

---

## 🧭 Purpose

This docs app helps us:

- Showcase TailYew components with live examples
- Document usage patterns and customization
- Catch layout bugs or UX issues early
- Provide a self-contained UI for users + contributors
- Enable easy static hosting (e.g. GitHub Pages)

---

## 🏗️ Tech Stack

- ⚙️ **Yew** – Rust-based frontend framework
- 🎨 **Tailwind CSS** – Utility-first styling
- 🧩 **TailYew** – Local component crate under development
- 🚚 **Trunk** – Build + serve the Yew app
- 🛠️ **Makefile** – Development workflow commands

---

## 📂 Project Layout

```
frontend/
├── static/            # Static assets (CSS, images, manifest, service worker)
├── dist/              # Trunk build output (gitignored)
├── src/               # Yew application source code
│   └── pages/         # Component-based route pages
├── main.css           # Tailwind input CSS
├── index.html         # Trunk entry point
├── Trunk.toml         # Trunk configuration
├── tailwind.config.js # Tailwind configuration
├── Makefile           # Build + serve commands
├── Cargo.toml         # Rust crate config
```

---

## 🚀 Local Development

This frontend is built and served with **Trunk**. Tailwind CSS is compiled via the Tailwind CLI (npm).

Prerequisites (once):

- `cargo install trunk`
- `rustup target add wasm32-unknown-unknown`
- `npm install` inside `frontend/` (for Tailwind CLI)

### 1. Build the frontend

```bash
make build
```

This does the following:

- Compiles Tailwind CSS to `static/styles.css`
- Runs `trunk build`, emitting the site into `dist/`

### 2. Start the dev server

```bash
make run
```

Opens your docs site at:

```
http://localhost:3031
```

### 3. Watch and rebuild when developing TailYew

If you’re editing components inside the `tailyew` crate:

```bash
make run-frontend
```

Trunk watches the frontend and `../crates/tailyew/src` for changes and hot-reloads the app.

---

## 🧼 Cleanup

```bash
make clean
```

Removes generated CSS and Trunk build artifacts.

---

## 🧪 Testing Components

Want to see how your new component looks or behaves?

1. Build it in `crates/tailyew/`
2. Import it into `frontend/src/pages/`
3. Add it to the docs view and rebuild
4. Tweak props and styles in isolation

This keeps development fast and focused.


## 📦 Dependency

This crate depends on your local `tailyew` component system:

```toml
# frontend/Cargo.toml
tailyew = { path = "../crates/tailyew" }
```

---

## 🛠️ Makefile Commands

Here are some useful commands:

```bash
make build           # Build CSS + WASM via Trunk
make run             # Serve the static site locally with Trunk
make clean           # Delete build output
```

Or from the root:

```bash
make run-frontend    # Start Trunk dev server (frontend + tailyew crate)
make fe-build        # Run `make build` inside frontend/
```

---

## 🙌 Contributing

If you're building or improving TailYew components, please use this frontend app to test and demonstrate your changes. This site is your canvas 🧑‍🎨

---

## 🔗 Related

- [TailYew Crate](../crates/tailyew/) – The core component library
- [Root Makefile](../Makefile) – Project-wide commands and orchestration
