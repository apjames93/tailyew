use tailyew::atoms::{Button, ButtonType, Section, TagType, Typo};
use tailyew::molecules::CodeBlock;
use yew::prelude::*;

#[function_component(GettingStarted)]
pub fn getting_started() -> Html {
    html! {
            <Section class="space-y-6">
                <Typo tag={TagType::H2}>{"✨ Getting Started with `create-tailyew-app`"}</Typo>
                <Typo tag={TagType::P}>{
                    "The easiest way to start building Rust frontends with TailYew is via our zero-config CLI."
                }</Typo>

                <Typo tag={TagType::H3}>{"🚀 1. Install the CLI"}</Typo>
                <Typo tag={TagType::P}>{"First, grab the scaffolding tool from crates.io:"}</Typo>
                <CodeBlock>
    {"cargo install create-tailyew-app"}
                </CodeBlock>

                <Typo tag={TagType::H3}>{"📦 2. Scaffold your new app"}</Typo>
                <Typo tag={TagType::P}>{"Run the CLI with your desired project name (and optional `--dest`):"}</Typo>
                <CodeBlock>
    {"create-tailyew-app my-awesome-app"}
                </CodeBlock>

                <Typo tag={TagType::P}>{
                    "This will create `./my-awesome-app/`, copy in a fully-working Yew + Tailwind starter, install Node dependencies, and set you up with hot-reload tooling."
                }</Typo>

                <Typo tag={TagType::H3}>{"⚙️ 3. Change into your project"}</Typo>
                <CodeBlock>
    {"cd my-awesome-app"}
                </CodeBlock>

                <Typo tag={TagType::H3}>{"🎬 4. Start the dev server"}</Typo>
                <Typo tag={TagType::P}>{
                    "You’ll be prompted whether to launch the dev server with hot-reload. If you skipped it, just run:"
                }</Typo>
                <CodeBlock>
                    {"make run-frontend"}
                </CodeBlock>

                <Typo tag={TagType::P}>{"Open "}
                    <code class="text-blue-500 font-mono">{"http://localhost:8080"}</code>
                    {" to see your new TailYew app in action!"}
                </Typo>

                <div class="pt-4">
                    <Button
                        button_type={ButtonType::Primary}
                        onclick={Callback::from(|_| {
                            let _ = web_sys::window()
                                .unwrap()
                                .open_with_url("https://github.com/apjames93/tailyew");
                        })}
                    >
                        {"Learn More on GitHub"}
                    </Button>
                </div>
            </Section>
        }
}
