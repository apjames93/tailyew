use tailyew::atoms::{Button, ButtonType, Section, TagType, Typo};
use yew::prelude::*;

#[function_component(GettingStarted)]
pub fn getting_started() -> Html {
    html! {
        <Section class="space-y-6">
            <Typo tag={TagType::H2}>{"✨ Getting Started with `create-tailyew-app`"}</Typo>
            <Typo tag={TagType::P}>{"The easiest way to start building Rust frontends with TailYew is by using our official starter template."}</Typo>

            <Typo tag={TagType::H3}>{"🚀 1. Clone the Starter"}</Typo>
            <pre class="bg-gray-800 text-white p-4 rounded overflow-x-auto text-sm">
                {"git clone https://github.com/apjames93/tailyew-starter your-app\ncd your-app"}
            </pre>

            <Typo tag={TagType::H3}>{"📦 2. Install Required Dependencies"}</Typo>
            <Typo tag={TagType::P}>{"TailYew apps use both Rust tooling and minimal Node.js utilities for building and serving your frontend."}</Typo>

            <Typo tag={TagType::H5}>{"Rust CLI tools:"}</Typo>
            <pre class="bg-gray-800 text-white p-4 rounded overflow-x-auto text-sm">
                {"cargo install wasm-pack cargo-watch\nbrew install binaryen"}
            </pre>
            <Typo tag={TagType::P}>{"These tools help compile your Yew app to WebAssembly (`wasm-pack`), enable hot reloading (`cargo-watch`), and optimize your `.wasm` binary (`binaryen`)."}</Typo>

            <Typo tag={TagType::H5}>{"JavaScript tools via npm:"}</Typo>
            <pre class="bg-gray-800 text-white p-4 rounded overflow-x-auto text-sm">
                {"npm install"}
            </pre>
            <Typo tag={TagType::P}>{"We install Tailwind CLI (for styling) and a static server (`serve`) to preview your app after build. This avoids needing a full Node bundler like Vite or Webpack."}</Typo>

            <Typo tag={TagType::H3}>{"🎨 3. Tailwind Setup"}</Typo>
            <Typo tag={TagType::P}>{"To make Tailwind CSS aware of all class usages in the TailYew crate, we copy the crate's source files into a local `vendor/` directory:"}</Typo>
            <pre class="bg-gray-800 text-white p-4 rounded overflow-x-auto text-sm">
                {"make copy-tailyew"}
            </pre>
            <Typo tag={TagType::P}>{"This ensures Tailwind picks up all dynamic and semantic utility classes when compiling your final CSS."}</Typo>

            <Typo tag={TagType::H3}>{"⚙️ 4. Build and Run Your App"}</Typo>
            <pre class="bg-gray-800 text-white p-4 rounded overflow-x-auto text-sm">
                {"make run-frontend"}
            </pre>
            <Typo tag={TagType::P}>{"This uses `cargo-watch` to hot-reload your Yew frontend as you save Rust files. Your Tailwind CSS is also recompiled with every change."}</Typo>

            <Typo tag={TagType::P}>{"Once it’s up, open "}
                <code class="text-blue-500 font-mono">{"http://localhost:8080"}</code>
                {" to see your TailYew-powered app in action."}
            </Typo>

            <div class="pt-4">
                <Button
                    button_type={ButtonType::Primary}
                    onclick={Callback::from(|_| {
                        let _ = web_sys::window().unwrap().open_with_url("https://github.com/apjames93/tailyew-starter");
                    })}
                >
                    {"View Starter on GitHub"}
                </Button>
            </div>
        </Section>
    }
}
