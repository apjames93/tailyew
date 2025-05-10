# ✨ TailYew
![Crates.io](https://img.shields.io/crates/v/tailyew)
![Downloads](https://img.shields.io/crates/d/tailyew)
![Docs.rs](https://img.shields.io/docsrs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)
![CI](https://github.com/apjames93/tailyew/actions/workflows/tailyew.yaml/badge.svg)
![WASM Ready](https://img.shields.io/badge/WASM-ready-blueviolet)
![Powered by Yew](https://img.shields.io/badge/Yew-0.21-blue)
![Components](https://img.shields.io/badge/components-50%2B-blue)
[![Demo](https://img.shields.io/badge/live-demo-0C66F0?logo=vercel&logoColor=white)](https://tailyew.com)

> A modern, reusable component library for [Yew](https://yew.rs) apps — powered by Tailwind CSS and written fully in Rust.

## 🚀 Why TailYew?

TailYew helps you build fast, beautiful, **dark-mode ready** Yew apps — with minimal styling effort.

- 🌗 **Dark mode** with Tailwind's `dark:` utilities
- 🧹 **Reusable components**: Buttons, Modals, Forms, Accordions, Tabs
- 📙 **Markdown rendering** Auto-maps to TailYew components (`Typo`, `A`, `Image`, `CodeBlock`) and supports dynamic `FormBuilder` blocks
- 📊 **Charts** — Canvas-based charts (Bar, Line, Bubble, Pie, Scatter) with no JS dependencies and theme-aware data visualizations
- 🧾 **Composable Forms** — Input-driven and config-driven forms with built-in state, modals, validation, and accessibility
- 🦠 **Pure Rust** — No JavaScript needed 
- 📦 **Small bundle sizes** — thanks to WASM and Tailwind JIT
- 📝 **A11y support** — ARIA roles, labels, and `aria-describedby` support (Ongoing improvement)

🔎 Explore live examples 👉 [https://tailyew.com/demo/getting_started](https://tailyew.com/demo/getting_started)

---

## 🧩 Component Coverage

> TailYew includes 50+ components. Below is a summary of a few key components— [see the full demo here »](https://tailyew.com)

- **Atoms**: `Button`, `Input`, `Textarea`, `Typo`, `Checkbox`, `Select`
- **Molecules**: `ModalButton`, `Popover`, `Accordion`, `Stepper`, `Markdown`
- **Organisms**: `Table`, `Form`, `NavBar`, `Sidebar`, `Card`, `HeroHeader`
- **Charts**: `BarChart`, `LineChart`, `BubbleChart`, `PieChart`, `ScatterPlot`
- **Forms**: Self-managed `Form` and `FormBuilder` with fully composable inputs, modal support, and accessible feedback

See all live: [https://tailyew.com](https://tailyew.com/)
---


# ⚡ Quick Start

## ✅ Option 1: Use the Starter Template (Recommended)

```bash
git clone https://github.com/apjames93/tailyew-starter my-app
cd my-app

cargo install wasm-pack cargo-watch
brew install binaryen
npm install # for tailwindcss and local js server for development

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
tailyew = "0.1.21"
```

---

## ⚡️ Important: Tailwind Setup (Safelist)

Tailwind CSS uses static analysis to determine which classes to include in your final CSS bundle. Since TailYew applies some classes dynamically, **you must safelist them**.

### 📅 Recommended: Use the Built-in Safelist HTML

TailYew includes a `tailyew-safelist.html` file that lists all runtime classes. Add it to your `tailwind.config.js`:

```js
module.exports = {
  darkMode: 'class',
  content: [
   './**/**/*.{html,js,rs}',
    './static/tailyew-safelist.html',
  ],
};
```

To copy the safelist file into your project:

```bash
mkdir -p vendor/tailyew
cp ~/.cargo/registry/src/*/tailyew-*/tailyew-safelist.html static/
```

### ⚠️ Alternative: Manually Add Classes

You can also manually define all TailYew utility classes in the `safelist` key inside `tailwind.config.js`. This approach is more error-prone and not recommended unless you're customizing heavily.

---

## 📂 TL;DR Setup

```bash
# Install TailYew
cargo add tailyew


cp ~/.cargo/registry/src/*/tailyew-*/tailyew-safelist.html static/

# Update Tailwind config
// tailwind.config.js
content: [
  './**/**/*.{html,js,rs}',
  './static/tailyew-safelist.html',
]

# Run your app
```

---

# 🛠️ Project Goals

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

MIT License — see [LICENSE](./LICENSE)

