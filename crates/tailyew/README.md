# 📙 TailYew – Reusable Yew + Tailwind Component Library

## 📜 Documentation

🧪 **Explore the live component docs and demos:**  
👉 [https://tailyew.com](https://tailyew.com)

See examples, props, and code for every component in the system.

---

## 🚀 Quick Start

To get up and running quickly, we recommend using our starter template.

### ✅ Clone the TailYew Starter

```bash
git clone https://github.com/apjames93/tailyew-starter your-app
cd your-app
```

### 📦 Install Tooling

You'll need a mix of Rust and Node tooling:

#### Rust CLI tools:

```bash
cargo install wasm-pack cargo-watch
brew install binaryen
```

- `wasm-pack`: Builds your Yew app to WebAssembly.
- `cargo-watch`: Watches your Rust files and rebuilds on changes.
- `binaryen`: Provides `wasm-opt` for optimizing `.wasm` output.

#### JavaScript tools (via `npm`):

```bash
npm install
```

- Installs `tailwindcss` (for styling) and `serve` (for previewing static builds).
- These tools are installed locally and used via `npx`.

### 🎨 Tailwind Setup

Because Tailwind requires static analysis to include only the used CSS classes, we run this command to expose TailYew’s class usage:

```bash
make copy-tailyew
```

This copies the crate's source code to `vendor/tailyew/`, ensuring Tailwind sees the actual classes used in our component definitions during build.

### ⚙️ Run Your Project

```bash
make run-frontend
```

This uses hot reloading (via `cargo-watch`) to update your app whenever your Yew code changes, and re-runs Tailwind when styles change.

Now visit 👉 [http://localhost:8080](http://localhost:8080) to see TailYew in action.

---

## 🌐 Live Docs

You can also view the same guide in the UI:  
👉 [https://tailyew.com/demo/getting_started](https://tailyew.com/demo/getting_started)

---

## 🏧 Highlighted Features

TailYew comes with battle-tested components including:

- ✅ **Forms** – Input, Select, Checkbox, JSON, Phone, File
- 📊 **Charts** – LineChart, BubbleChart (canvas-based) 
- 🧪 **Modals, Accordions, AppBar, Tabbed Interfaces**
- 🔐 **Theme support** with system/light/dark and utility-first color tokens
- ✂️ **Clipboard, Notifications, Popovers**, and more

Explore all components at 👉 [https://tailyew.com/demo](https://tailyew.com/demo)

---

## 🏓️ Project Goals

- 💡 **Atomic Design** pattern (atoms → molecules → organisms)
- ⚙️ **Yew-native** components — idiomatic Rust, no JavaScript
- 🎨 **Tailwind-first** styling — utility-based, flexible, consistent
- 🧪 **Well-documented** components
- 📙 **Composable API** for building custom UIs

---

## 📁 Folder Structure

```bash
crates/tailyew/
├── src/
│   ├── atoms/         # Low-level building blocks (buttons, inputs, etc.)
│   ├── molecules/     # Mid-level UI (forms, modals, selects)
│   ├── organisms/     # Full UI blocks (headers, footers, nav)
│   ├── charts/        # Data visualization components (optional)
│   ├── form/          # Form layout & state handling helpers
│   ├── icons/         # Custom SVG icon components
│   └── lib.rs         # Exports all components and modules
├── Makefile           # Dev commands (build, docs)
├── Cargo.toml         # Rust crate manifest
└── README.md          # You are here
```

## 📌 Related

- [Yew Framework](https://yew.rs/)
- [Tailwind CSS](https://tailwindcss.com/)
- [TailYew Starter](https://github.com/apjames93/tailyew-starter)
