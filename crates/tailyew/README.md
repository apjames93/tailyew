![Crates.io](https://img.shields.io/crates/v/tailyew)
![Docs.rs](https://img.shields.io/docsrs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)

# 🌟 TailYew
> A modern, reusable component library for [Yew](https://yew.rs) apps — powered by Tailwind CSS and built in Rust.

TailYew helps you ship fast, beautiful Yew apps using fully-styled, accessible components with dark mode, markdown rendering, charts, modals, and form elements — all built in idiomatic Rust.

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

## 📸 Preview

> Explore TailYew’s live components, props, and code samples:  
👉 [https://tailyew.com](https://tailyew.com)

---

> ## 🚀 Get Started in 5 Minutes  
> ...*if you already have all the dependencies, a stable toolchain, and emotionally prepared for Rust*

### ✅ Option 1: Use the Starter Template (Recommended)

```bash
git clone https://github.com/apjames93/tailyew-starter my-app
cd my-app

cargo install wasm-pack cargo-watch
brew install binaryen
npm install # for local deps tailwindcss cli and serve

make hot-run
```

Then open 👉 [http://localhost:8080](http://localhost:8080)

This gives you hot reloading, built-in Tailwind support, and access to every TailYew component.

---

## 📦 Using TailYew as a Crate (without starter)

Add to your `Cargo.toml`:

```toml
tailyew = "0.1.2"
```

### 🎨 Tailwind Setup (Important!)

Tailwind uses static analysis to detect classes. 
To expose classes from TailYew's crate, we have the make comand:

```bash
make copy-tailyew
```

This copies `crates/tailyew` into your app’s `vendor/` directory so Tailwind includes all component class usage.
This is ran with `make run-frontend`

Make sure your `tailwind.config.js` includes the path:

```js
content: [
  "./src/**/*.rs", // <---- for other components that you will build in your project
  "../crates/tailyew/src/**/*.rs", // <-- for TailYew usage
],
```

---

Then use in your app:

```rust
use tailyew::atoms::Button;

html! {
  <Button>{ "Click Me!" }</Button>
}
```

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

## 🤝 Contributing

TailYew is an open source project — and we’d love your help!

### ✨ Ways to Contribute

- Submit a **feature request** or improvement idea
- Report **bugs or visual issues**
- Build new components or improve styling
- Add or update documentation and demos

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
