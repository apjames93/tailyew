use tailyew::atoms::{Button, ButtonType, Section, TagType, Typo};
use tailyew::molecules::CodeBlock;
use yew::prelude::*;

#[component(GettingStarted)]
pub fn getting_started() -> Html {
    html! {
        <Section class="space-y-6">
            <iframe
                src="https://ghbtns.com/github-btn.html?user=apjames93&repo=tailyew&type=star&count=true&size=large"
                frameborder="0"
                scrolling="0"
                width="160"
                height="30"
                title="GitHub"
            />
            <Typo tag={TagType::H2}>{"✨ Getting Started with `create-tailyew-app`"}</Typo>

            <Typo tag={TagType::P}>
                {"The easiest way to start building Rust frontends with TailYew is via our zero-config CLI."}
            </Typo>

            <Typo tag={TagType::H3}>{"🚀 1. Install the CLI"}</Typo>
            <Typo tag={TagType::P}>{"First, grab the scaffolding tool from crates.io:"}</Typo>
            <CodeBlock>{"cargo install create-tailyew-app"}</CodeBlock>

            <Typo tag={TagType::P} class="text-yellow-700 dark:text-yellow-300 italic">
                {"Note: Tailwind CSS requires `npm` to be installed locally. It’s only used during development to build your styles."}
            </Typo>

            <Typo tag={TagType::H3}>{"📦 2. Scaffold your new app"}</Typo>
            <Typo tag={TagType::P}>{"Run the CLI with your desired project name (and optional `--dest`):"}</Typo>
            <CodeBlock>{"create-tailyew-app my-awesome-app"}</CodeBlock>

            <Typo tag={TagType::P}>
                {"This will create `./my-awesome-app/`, copy in a fully-working Yew + Tailwind starter, install Node dependencies, and set you up with hot-reload tooling."}
            </Typo>

            <Typo tag={TagType::H3}>{"⚙️ 3. Change into your project"}</Typo>
            <CodeBlock>{"cd my-awesome-app"}</CodeBlock>

            <Typo tag={TagType::H3}>{"🎬 4. Start the dev server"}</Typo>
            <Typo tag={TagType::P}>
                {"You’ll be prompted whether to launch the dev server with hot-reload. If you skipped it, just run:"}
            </Typo>
            <CodeBlock>{"make run-frontend"}</CodeBlock>

            <Typo tag={TagType::P}>
                {"Open "}
                <code class="text-blue-500 font-mono">{"http://localhost:9001"}</code>
                {" to see your new TailYew app in action!"}
            </Typo>

            <Typo tag={TagType::P}>
                {"Looking for the TailYew crate? Visit us on "}
                <a
                    href="https://crates.io/crates/tailyew"
                    target="_blank"
                    class="underline text-blue-500 hover:text-blue-700"
                >
                    {"crates.io"}
                </a>
                {" for version info and metadata."}
            </Typo>

            <div class="pt-4">
                <Button
                    button_type={ButtonType::Primary}
                    on_click={Callback::from(|_| {
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
