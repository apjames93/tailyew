# 🌟 TailYew
> A modern, reusable component library for [Yew](https://yew.rs) apps — powered by Tailwind CSS and built in Rust.

TailYew helps you ship fast, beautiful Yew apps using fully-styled, accessible components with dark mode, markdown rendering, charts, modals, and form elements — all built in idiomatic Rust.

---

## 📸 Preview

> Explore TailYew’s live components, props, and code samples:  
👉 [https://tailyew.com](https://tailyew.com)

---

## 🚀 Get Started in 2 Minutes

### ✅ Option 1: Use the Starter Template (Recommended)

```bash
git clone https://github.com/apjames93/tailyew-starter my-app
cd my-app
make run-frontend
```

Then open 👉 [http://localhost:8080](http://localhost:8080)

This gives you hot reloading, built-in Tailwind support, and access to every TailYew component.

---

### 🛠️ Install Tooling

> You'll need a mix of Rust + JS tooling:

#### Rust CLI tools:

```bash
cargo install wasm-pack cargo-watch
brew install binaryen
```

#### JavaScript tools:

```bash
npm install
```

> Installs `tailwindcss` and dev tools (e.g. `serve`)

---

### 🎨 Tailwind Setup (Important!)

Tailwind uses static analysis to detect classes. To expose classes from TailYew's crate, run:

```bash
make copy-tailyew
```

This copies `crates/tailyew` into your app’s `vendor/` directory so Tailwind includes all component class usage.

---

### ⚙️ Run the App

```bash
make run-frontend
```

This watches both Tailwind + Yew code for hot reloading using `cargo-watch`.

---

## 📦 Using TailYew as a Crate (without starter)

Add to your `Cargo.toml`:

```toml
tailyew = "0.1.2"
```

Make sure your `tailwind.config.js` includes the path:

```js
content: [
  "./src/**/*.rs",
  "../crates/tailyew/src/**/*.rs", // <-- for TailYew usage
],
```

Then use in your app:

```rust
use tailyew::atoms::Button;

html! {
  <Button>{ "Click Me!" }</Button>
}
```

---

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

## 🏗️ Project Goals

- 💡 **Atomic Design** – Atoms → Molecules → Organisms
- ⚙️ **Yew-Native** – Fully idiomatic Rust, zero JavaScript
- 🎨 **Tailwind-First** – Utility classes, theming, dark mode
- 📚 **Composable APIs** – Clean prop-driven ergonomics
- 🧪 **Developer Experience** – Hot reload, clear docs, consistent props

---

## 📁 Folder Structure

```bash
crates/tailyew/
├── src/
│   ├── atoms/         # Buttons, Inputs, etc.
│   ├── molecules/     # Modals, Forms, Selects
│   ├── organisms/     # Full blocks like Nav, Footer
│   ├── charts/        # Canvas-based charts (optional)
│   ├── form/          # Form layout + state helpers
│   ├── icons/         # Reusable SVGs
│   └── lib.rs         # Exports all public components
├── Makefile           # Dev commands (build, docs, hot reload)
├── Cargo.toml         # Rust crate config
└── README.md          # You're here
```

---

## 🔗 Related

- 🦀 [Yew Framework](https://yew.rs/)
- 🎨 [Tailwind CSS](https://tailwindcss.com/)
- 🚀 [TailYew Starter](https://github.com/apjames93/tailyew-starter)

---

> Looking to build a full Yew app with Markdown, forms, charts, and SSR-ready UI blocks? TailYew is your foundation.
