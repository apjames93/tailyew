Sure! Here’s a **developer-focused Markdown blog post** introducing TailYew and walking through the architecture, dev workflow, and what’s next.

---

# 🧩 Introducing TailYew: A Reusable Component System for Yew + Tailwind

> Build fast. Stay elegant. Write frontend code in Rust — the way it should feel.

---

## 📚 Table of Contents

- [What is TailYew?](#what-is-tailyew)
- [Why We Built It](#why-we-built-it)
- [Project Structure](#project-structure)
- [Splitting the Crate and Docs App](#splitting-the-crate-and-docs-app)
- [The Power of Makefile Workflows](#the-power-of-makefile-workflows)
- [How Hot Reloading Works](#how-hot-reloading-works)
- [Why Yew + Tailwind](#why-yew--tailwind)
- [How We Document with Real Usage](#how-we-document-with-real-usage)
- [What’s Coming Next](#whats-coming-next)

---

## What is TailYew?

**TailYew** is an open-source component system for Rust’s [Yew](https://yew.rs) frontend framework, styled entirely with [Tailwind CSS](https://tailwindcss.com). It’s designed to help Rust developers ship **elegant, consistent, and accessible UIs** — fast.

At its core, TailYew is:

- A reusable crate: `tailyew` — with Yew components built using Atomic Design
- A Yew-powered docs app: a live demo site, dogfooding environment, and static docs playground
- A unified Cargo workspace: orchestrated with a Makefile for a smooth DX

---

## Why We Built It

Building Yew apps from scratch is empowering — but not always ergonomic.

We found ourselves rewriting the same form inputs, buttons, and layout containers across projects. We needed:

- Reusable, Tailwind-styled components
- Consistent UX across products
- A **real Yew-native solution** — no Web Components, no JS frameworks
- A development flow that felt as productive as modern JS (but in Rust)

So we built TailYew.

---

## Project Structure

Here’s how our workspace is laid out:

```
.
├── crates/
│   └── tailyew/        # The reusable component system crate
├── frontend/           # A Yew-powered docs app using TailYew
├── docs/               # Markdown docs (e.g. GitHub Pages)
├── Makefile            # Top-level dev orchestrator
```

Each part has a purpose:

- `crates/tailyew`: the core reusable library, structured by **Atomic Design** (`atoms/`, `molecules/`, `organisms/`, etc.)
- `frontend/`: a full Yew app that showcases components with live examples and docs
- `Makefile`: acts as our dev CLI — for builds, tests, docs, and CI

---

## Splitting the Crate and Docs App

We intentionally split the system into two parts:

### ✅ `crates/tailyew/` (the library)

- Pure component logic
- Styled with Tailwind, no app-specific logic
- Easy to publish on [crates.io](https://crates.io)

### 🎨 `frontend/` (the playground)

- Uses the crate as a dependency
- Live renders components with props
- Documents usage, customization, accessibility
- Hosted as a static site (e.g. GitHub Pages)

This separation keeps our crate clean, while giving us a place to **dogfood and test** every change.

---

## The Power of Makefile Workflows

Rather than repeating `cargo build` and `wasm-pack` commands, we standardized dev with a **top-level Makefile**:

```bash
# 🚀 Launch the frontend docs site
make run-frontend

# 🧪 Run all tests
make test

# 📚 Watch crate docs live
make watch-docs

# 📦 Build the TailYew crate
make build-tailyew

# 🛰️ Publish the crate
make publish-tailyew
```

Each command aliases deeper workspace actions:

- `make run-frontend` watches **both** the crate and frontend
- `make fe-build`, `make tailyew-doc`, etc. delegate to subfolders
- `make clean` wipes generated CSS + WASM

This gives contributors a single, consistent CLI for all dev tasks.

---

## How Hot Reloading Works

We use `cargo watch` + `wasm-pack` + `serve` to enable **live-reloading** across the system:

```bash
# One command to rule them all:
make run-frontend
```

This will:

- Rebuild TailYew whenever a file changes in `crates/tailyew/src`
- Recompile the Yew frontend (WASM + Tailwind) on file changes
- Serve the docs app at `http://localhost:3030`
- Reflect changes almost instantly in the browser

The result: a seamless dev loop with live editing, testing, and feedback.

---

## Why Yew + Tailwind?

We love this stack because it feels:

- **Native** – write your UI in Rust, not in JS wrappers
- **Performant** – compile to WebAssembly for speed
- **Composable** – build small, powerful building blocks
- **Style-safe** – Tailwind's utility-first CSS keeps our components consistent, accessible, and easy to theme

We’re committed to keeping TailYew idiomatic to Yew — no JS interop, no hacks, just Rust + HTML + Tailwind.

---

