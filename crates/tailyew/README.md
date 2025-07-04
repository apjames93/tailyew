# ✨ TailYew  
> A modern, reusable component library for [Yew](https://yew.rs) apps — powered by Tailwind CSS and written fully in Rust.

[![Crates.io](https://img.shields.io/crates/v/tailyew)](https://crates.io/crates/tailyew)
![Downloads](https://img.shields.io/crates/d/tailyew)
[![Docs.rs](https://img.shields.io/docsrs/tailyew)](https://docs.rs/tailyew)
![License](https://img.shields.io/crates/l/tailyew)
[![CI](https://github.com/apjames93/tailyew/actions/workflows/tailyew.yaml/badge.svg)](https://github.com/apjames93/tailyew/actions)
![WASM Ready](https://img.shields.io/badge/WASM-ready-blueviolet)
![Powered by Yew](https://img.shields.io/badge/Yew-0.21-blue)
![Components](https://img.shields.io/badge/components-50%2B-blue)
[![Demo](https://img.shields.io/badge/live-demo-0C66F0?logo=vercel&logoColor=white)](https://tailyew.com)

---

## 🚀 Why TailYew?

TailYew helps you build fast, beautiful, **dark-mode ready** Yew apps — with minimal styling effort.

- 🌗 **Dark mode** with Tailwind's `dark:` utilities  
- 🧹 **Reusable components**: Buttons, Modals, Forms, Accordions, Tabs  
- 📙 **Markdown rendering** auto-maps to TailYew components (`Typo`, `A`, `Image`, `CodeBlock`) and supports embedded `FormBuilder` blocks  
- ✍️ **Dynamic Forms in Markdown** — render live `FormBuilder` blocks from fenced code like ````form ...```` via `pulldown-cmark`  
- 📊 **Charts** — Canvas-based charts (Bar, Line, Bubble, Pie, Scatter) with no JS dependencies and theme-aware data visualizations  
- 🧾 **Composable Forms** — Input-driven and config-driven forms with built-in state, modals, validation, and accessibility  
- 🔁 **Async Forms** — All forms support `FormSubmitCallback` with loading, error, and success handling built in  
- 🦠 **Pure Rust** — No JavaScript needed  
- 📦 **Small bundle sizes** — thanks to WASM and Tailwind JIT  
- 📝 **A11y support** — ARIA roles, labels, and `aria-describedby` support  

🔎 Explore live examples 👉 [https://tailyew.com/demo/getting_started](https://tailyew.com/demo/getting_started)

---

## 🧩 Component Coverage

> TailYew includes 50+ components. Below is a summary of a few key components— [see the full demo here »](https://tailyew.com)

- **Atoms**: `Button`, `Input`, `Textarea`, `Typo`, `Checkbox`, `Select`  
- **Molecules**: `ModalButton`, `Popover`, `Accordion`, `Stepper`, `Markdown`, `Notification`  
- **Organisms**: `Table`, `Form`, `NavBar`, `Sidebar`, `Card`, `HeroHeader`  
- **Charts**: `BarChart`, `LineChart`, `BubbleChart`, `PieChart`, `ScatterPlot`  
- **Forms**: Self-managed `Form` and `FormBuilder` with fully composable inputs, modal support, and accessible feedback  

See all live: [https://tailyew.com](https://tailyew.com/)

---

> If you like this project, consider giving it a ⭐ — it helps others discover TailYew!  
[![Star](https://img.shields.io/github/stars/apjames93/tailyew?style=social)](https://github.com/apjames93/tailyew/stargazers)

---

## ⚡ Quick Start

### ✅ Option 1: Scaffold via CLI (Recommended)

We now provide a zero-config CLI to bootstrap a TailYew SPA in seconds:

```bash
# 1) Install the scaffolding tool
cargo install create-tailyew-app

# 2) Scaffold your project (this creates `my-app/` for you)
create-tailyew-app my-app

# 3) Change into it
cd my-app

# 4) Start the dev server
make run-frontend
`````

You’ll get:

* A fully working Yew/WASM + Tailwind starter
* `npm install` already run for you
* `cargo-watch` installed for hot-reload
* A prompt (or `make run-frontend`) to launch at [http://localhost:9001](http://localhost:9001)
* A smart `Makefile` with dev commands: `run-frontend`, `fe-check`, `pretty`, and more

---

### ✅ Option 2: Add TailYew to Your Own App

In your `Cargo.toml`:

```toml
tailyew = "0.1.38"
```

---

## ⚡️ Important: Tailwind Setup (Safelist)

Tailwind CSS uses static analysis to determine which classes to include in your final CSS bundle. Since TailYew applies some classes dynamically, **you must safelist them**.

### Recommended: Use the Built-in Safelist HTML

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

# Copy the safelist
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

## ♿ Accessibility

TailYew is committed to accessible UI components:

* All form inputs support `aria-label`, `aria-describedby`, and semantic labels
* Modals and alerts use correct ARIA roles
* Live validation feedback is announced for screen readers
* Form errors are keyboard-navigable and styled consistently
* A11y is an ongoing area of improvement — issues and PRs welcome!

---

# 🛠️ Project Goals

| Goal                      | Status                          |
| ------------------------- | ------------------------------- |
| 🧹 Atomic Components      | ✅ Atoms → Molecules → Organisms |
| ⚙️ Yew-Native Rust Code   | ✅ No JavaScript needed          |
| 🎨 Tailwind-First Styling | ✅ Utility-first classes         |
| 🌗 Dark Mode Friendly     | ✅ Fully supported               |
| 📙 Typed Prop APIs        | ✅ Rust ergonomics               |

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

* Suggest a new component
* Improve UX/UI or theming
* Add missing dark mode styles
* Write docs or demos
* Report bugs and issues

**Before submitting a PR:**

* Run `make release-check`
* Test both **light mode** and **dark mode**
* Include **before/after screenshots** if you change visuals

---

# 📂 Helpful Links

* 📥 [Open a Pull Request](https://github.com/apjames93/tailyew/compare)
* 🐛 [Report a Bug](https://github.com/apjames93/tailyew/issues/new?template=bug_report.md)
* 💡 [Propose a Feature](https://github.com/apjames93/tailyew/issues/new?template=feature_request.md)

---

# 🔗 Related Projects

* 🦠 [Yew Framework](https://yew.rs/)
* 🎨 [TailwindCSS](https://tailwindcss.com/)
* 🚀 [Create TailYew App](https://crates.io/crates/create-tailyew-app)

---

👌 Thank you for helping grow the Rust UI ecosystem!

---

# 📜 License

MIT License — see [LICENSE](./LICENSE)
