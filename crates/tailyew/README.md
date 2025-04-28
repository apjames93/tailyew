---

# 🌟 TailYew

> A modern, reusable component library for [Yew](https://yew.rs) apps — powered by Tailwind CSS and written fully in Rust.

![Crates.io](https://img.shields.io/crates/v/tailyew)
![Docs.rs](https://img.shields.io/docsrs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)

---

## 🚀 Why TailYew?

TailYew helps you build fast, beautiful, **dark-mode ready** Yew apps — with minimal styling effort.

- 🌗 **Dark mode** with Tailwind's `dark:` utilities
- 🧹 **Reusable components**: Buttons, Modals, Forms, Accordions, Tabs
- 📙 **Markdown rendering** with sanitization
- 📊 **Charts** (Canvas-based Bubble and Line charts)
- ✂️ **Clipboard, Popovers, Notifications, and more**
- 🦠 **Pure Rust** — No JavaScript needed
- 📦 **Small bundle sizes** — thanks to WASM and Tailwind JIT

🔎 Explore live examples 👉 [https://tailyew.com/demo/getting_started](https://tailyew.com/demo/getting_started)

---

## 📸 Preview

✨ See demos, props, and usage:  
👉 [https://tailyew.com](https://tailyew.com)

---

# 🧹 Quick Start

## ✅ Option 1: Use the Starter Template (Recommended)

```bash
git clone https://github.com/apjames93/tailyew-starter my-app
cd my-app

cargo install wasm-pack cargo-watch
brew install binaryen
npm install

make hot-run
```

Then open 👉 [http://localhost:8080](http://localhost:8080)

You get:

- Hot reloading
- Preconfigured TailwindCSS
- All TailYew components ready to use

---

## ✅ Option 2: Add TailYew to Your Own App

In your `Cargo.toml`:

```toml
tailyew = "0.1.13"
```

### ⚡️ Important: Tailwind Setup

Because Tailwind uses static analysis, you must **expose TailYew's classes** manually.

We include a **safelist** file (`tailyew-safelist.html`) inside the crate.

Your `tailwind.config.js` should look like:

```js
darkMode: 'class',
content: [
  './src/**/*.rs',
  './**/*.html',
  './js/**/*.js',
  './components/**/*.{html,js,rs}',
  './vendor/tailyew/**/*.rs',               // 👉 TailYew source files
  './vendor/tailyew/tailyew-safelist.html',  // 👉 TailYew critical runtime classes
],
```

### If you are using **TailYew Starter**:

Simply run:

```bash
make copy-tailyew
```

✅ This will copy TailYew sources + safelist automatically.

### If you are **not** using the starter:

Manually copy TailYew files:

```bash
mkdir -p vendor/tailyew
cp -r ~/.cargo/registry/src/*/tailyew-*/src vendor/tailyew/
cp ~/.cargo/registry/src/*/tailyew-*/tailyew-safelist.html vendor/tailyew/
```

(Adjust the exact path to your system.)

---

# 🏗️ Project Goals

| Goal                     | Status |
| ------------------------ | ------ |
| 🧹 Atomic Components      | ✅ Atoms → Molecules → Organisms |
| ⚙️ Yew-Native Rust Code    | ✅ No JavaScript needed |
| 🎨 Tailwind-First Styling  | ✅ Utility-first classes |
| 🌗 Dark Mode Friendly      | ✅ Fully supported |
| 📙 Typed Prop APIs         | ✅ Rust ergonomics |

---

# 📁 Folder Structure

```bash
crates/tailyew/
├── src/
│   ├── atoms/         # Low-level UI primitives (Button, Input, etc.)
│   ├── molecules/     # Combined components (Form, ModalButton, etc.)
│   ├── organisms/     # Full page structures (Navbar, Footer, Table)
│   ├── charts/        # Canvas-based charts
│   ├── form/          # Form helpers and layout
│   └── lib.rs         # Top-level exports
├── Cargo.toml         # Crate config
├── Makefile           # Dev commands (build, release-check, docs)
└── tailyew-safelist.html # ✨ Tailwind runtime classes (critical)
```

---

# 🤝 Contributing

We welcome contributions! ❤️

**Ways you can help:**

- Suggest a new component
- Improve UX/UI or theming
- Add missing dark mode styles
- Write docs or demos
- Report bugs and issues

**Before submitting a PR:**

- Run `make release-check`
- Test both **light mode** and **dark mode**
- Include **before/after screenshots** if you change visuals

---

# 📂 Helpful Links

- 📥 [Open a Pull Request](https://github.com/apjames93/tailyew/compare)
- 🐛 [Report a Bug](https://github.com/apjames93/tailyew/issues/new?template=bug_report.md)
- 💡 [Propose a Feature](https://github.com/apjames93/tailyew/issues/new?template=feature_request.md)

---

# 🔗 Related Projects

- 🦠 [Yew Framework](https://yew.rs/)
- 🎨 [TailwindCSS](https://tailwindcss.com/)
- 🚀 [TailYew Starter](https://github.com/apjames93/tailyew-starter)

---

👌 Thank you for helping grow the Rust UI ecosystem!

---

# 📜 License

MIT License — see [LICENSE](./LICENSE).

---

# 🎯 TL;DR for Setup

```bash
# Install TailYew
cargo add tailyew

# Copy TailYew files (if not using starter)
mkdir -p vendor/tailyew
cp -r ~/.cargo/registry/src/*/tailyew-*/src vendor/tailyew/
cp ~/.cargo/registry/src/*/tailyew-*/tailyew-safelist.html vendor/tailyew/

# Add to tailwind.config.js
'./vendor/tailyew/**/*.rs',
'./vendor/tailyew/tailyew-safelist.html',

# Start building 🚀
```

