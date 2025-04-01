<!-- README.md -->
# 🌬️ TailYew

**TailYew** is a component system for building elegant, modern UI in Rust using [Yew](https://yew.rs/) and [Tailwind CSS](https://tailwindcss.com/).

This repository includes:

- 📦 `tailyew` – A reusable crate of Yew + Tailwind components
- 🌐 `frontend` – A live documentation site that uses TailYew internally
- 🛠️ A Makefile-powered workflow for building, testing, and documenting

---

## 📚 Why TailYew?

We love Rust and wanted a modern, type-safe UI library that makes building frontends with Yew feel like working in systems like React + Tailwind — but with all the benefits of Rust.

**TailYew is:**

- 🔩 Composable: Built with Atomic Design principles (atoms → organisms)
- 💅 Styled: Powered by Tailwind for utility-first control
- 📦 Packaged: Designed as a crate you can use in any Yew project
- 🧪 Tested: Built and tested inside a real Yew app (the docs site)
- 🧰 Developer-focused: Easy workflows via `make`, `cargo-watch`, `wasm-pack`

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
# 🧪 Run all tests
make test

# 📐 Format and lint
make pretty

# 🔁 Rebuild frontend docs on crate or UI changes
make run-frontend

# 📚 Watch and rebuild Rust API docs
make watch-docs

# 📦 Publish the component crate
make publish-tailyew
```

You can also run targets inside subdirectories:

```bash
make fe-build         # Run target from frontend/Makefile
make tailyew-doc      # Run target from crates/tailyew/Makefile
```

---

## 🧱 Using TailYew in Your Project

```toml
# In your Cargo.toml
tailyew = "0.1"
```

Or use the latest version from GitHub (until it's published):

```toml
tailyew = { git = "https://github.com/your-org/tailyew" }
```

Then, in your Yew app:

```rust
use tailyew::atoms::Button;

html! {
  <Button label="Click Me!" />
}
```


## 🤝 Contributing

We welcome issues, suggestions, and pull requests! Feel free to open a discussion if you’re using TailYew and want to share feedback or ideas.

