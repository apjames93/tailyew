# create-tailyew-app

**A zero-config CLI to scaffold and bootstrap a TailYew frontend SPA.**

`create-tailyew-app` embeds a fully-working Yew + Tailwind starter template and automates:

1. Copying the starter files into your target directory  
2. Renaming placeholder Cargo manifests  
3. Running `npm install` in the `frontend/` sub-folder  
4. Installing `cargo-watch` for hot-reloads  
5. Offering to launch your dev server with `make run-frontend`

---

## 🛠️ Installation

```bash
cargo install create-tailyew-app
````

> Requires Rust 1.65+ and `npm` on your PATH.

---

## 🚀 Usage

```bash
create-tailyew-app <NAME> [--dest <BASE_DIR>]
```

* **`NAME`** (required)
  The name of your new project folder.
* **`--dest`** (optional, defaults to `.`)
  Base directory in which to create `NAME`.
  If it doesn’t exist, it will be created.

### Examples

```bash
# Scaffold into ./my-app
create-tailyew-app my-app

# Scaffold into /tmp/foo/my-app
create-tailyew-app my-app --dest /tmp/foo
```

---

## 📁 What’s generated

```text
<BASE_DIR>/<NAME>/
├─ frontend/
│  ├─ Cargo.toml           # Yew/WASM crate
│  ├─ tailwind.config.js   # TailwindCSS setup
│  ├─ src/                 # Rust & Tera templates
│  ├─ static/              # HTML, CSS, images, WASM pkg
│  └─ Makefile             # frontend build & serve tasks
├─ Makefile                # root orchestrator (run-frontend, lint, format)
└─ README.md               # this file
```

---

## 🎬 Dev server

After scaffolding:

```bash
cd <BASE_DIR>/<NAME>
make run-frontend
```

This will:

1. Watch your Rust sources (`frontend/src/…`)
2. Rebuild & repackage the WASM on changes
3. Reload the browser automatically

By default, it serves at `http://localhost:8080`.

---

## 🔧 Other Makefile targets

From your project root:

* **`make format`** — `cargo fmt --all`
* **`make lint`** — `cargo clippy --workspace --all-targets --all-features`
* **`make pretty`** — `make format && make lint`
* **`make fe-build`** — build the Yew/WASM frontend
* **`make fe-check`** — `cargo check` on `frontend/`
* **`make fe-run`** — serve with hot-reload (used by `run-frontend`)

---

## 📦 Publishing & Crate.io

This CLI is published on [crates.io](https://crates.io/crates/create-tailyew-app).
Before you publish your own changes, bump the `version` in `Cargo.toml` and run:

```bash
cargo publish
```

---

## ❤️ Contributing

1. Fork & clone this repo
2. Update `starter/app/` to modify the template
3. Tweak CLI behavior in `src/`
4. Run `make pretty` to format & lint
5. Open a PR — all feedback welcome!

---

# 📜 License

MIT License — see [LICENSE](./LICENSE)
