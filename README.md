![Crates.io](https://img.shields.io/crates/v/tailyew)
![Docs.rs](https://img.shields.io/docsrs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)

# 🌬️ TailYew
> A modern, reusable component library for [Yew](https://yew.rs) apps — powered by Tailwind CSS and built in Rust.

TailYew helps you ship fast, beautiful Yew apps using fully-styled, accessible components with dark mode, markdown rendering, charts, modals, and form elements — all built in idiomatic Rust.

## 🧩 Component Highlights

TailYew comes with battle-tested components including:

- ✅ **Forms** – Input, Select, Checkbox, JSON, Phone, File
- 📊 **Charts** – LineChart, BubbleChart (canvas-based)
- 🧪 **Modals, Accordions, AppBar, Tabs**
- 📙 **Markdown** – Parse + sanitize with `pulldown-cmark` and `ammonia`
- ✂️ **Clipboard**, **Notifications**, **Popover**, **Section**, and more
- 🌗 **Dark mode ready** — with Tailwind’s `dark:` utilities

🔎 See them all at 👉 [https://tailyew.com/demo/getting_started](https://tailyew.com/demo/getting_started)

---

## 🧱 Using TailYew in Your Project

Check out our [starter kit](https://github.com/apjames93/tailyew-starter) to get up and running quickly

If you're integrating manually or want to poke around, start with our [crate README](https://github.com/apjames93/tailyew/blob/main/crates/tailyew/README.md) for install instructions and usage examples.

> Bonus: The starter kit is the fastest path to a working TailYew setup. The README is your DIY route if you're feeling brave. Either way, you've got options.

## 📚 Why TailYew?

A modern, type-safe UI library that makes building frontends with Yew feel like working in systems like React + Tailwind — but with all the benefits of Rust.

**TailYew is:**

- 🔩 Composable: Built with Atomic Design principles (atoms → organisms)
- 💅 Styled: Powered by Tailwind for utility-first control
- 📦 Packaged: Designed as a crate you can use in any Yew project
- 🧪 Tested: Built and tested inside a real Yew app in [the docs site](https://tailyew.com/demo/getting_started)
- 🧰 [Tailyew Starter](https://github.com/apjames93/tailyew-starter) kit Developer-focused: Easy workflows via `make`, `cargo-watch`, `wasm-pack`

---

## 🗂️ Project Structure

```
.
├── crates/
│   └── tailyew/        # The reusable component library
├── frontend/           # The docs & demo site (real Yew app)
├── docs/               # Optional GitHub Pages markdown
├── Makefile            # Root Makefile for all workflows
```

Inside `tailyew/src/`, we organize components by **Atomic Design**:

```
atoms/       # Basic elements like Input, Button
molecules/   # Input groups, Select, etc.
organisms/   # Layouts and multi-part components
charts/      # Canvas & chart-based elements
form/        # Form wrappers and logic
icons/       # Shared SVG icon components
```

---

## ⚙️ Developer Workflow

Everything is wired up with Makefiles — no guessing.

```bash
# 📐 Format and lint
make pretty

# 🔁 Rebuild frontend docs on crate or UI changes
make run-frontend

# 📚 Watch and rebuild Rust API docs
make watch-docs

```

You can also run targets inside subdirectories:

```bash
make fe-build         # Run target from frontend/Makefile
make tailyew-doc      # Run target from crates/tailyew/Makefile
```
---

### ✅ Contribution Checklist

Before submitting a pull request:

- Make sure `make release-check` passes
- If it's a visual/UI change, include **before/after screenshots**
- Update or add related demo pages in `frontend/src/pages/`
- Test in both **light mode** and **dark mode**

---

### 📄 Helpful Links

- 📥 [Open a Pull Request](https://github.com/apjames93/tailyew/compare)
- 🐛 [Report a Bug](https://github.com/apjames93/tailyew/issues/new?template=bug_report.md)
- 💡 [Propose a Feature](https://github.com/apjames93/tailyew/issues/new?template=feature_request.md)

---

🙌 Thank you for helping grow the Rust UI ecosystem!

---

## 🔗 Related

- 🦀 [Yew Framework](https://yew.rs/)
- 🎨 [Tailwind CSS](https://tailwindcss.com/)
- 🚀 [TailYew Starter](https://github.com/apjames93/tailyew-starter)

---
