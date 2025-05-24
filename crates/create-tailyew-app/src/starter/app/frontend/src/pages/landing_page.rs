use tailyew::atoms::{Section, Spacer, TagType, Typo};
use tailyew::molecules::{Accordion, HeroHeader};
use tailyew::organisms::{Card, TabItem, Tabs};
use yew::prelude::*;

const HERO_IMAGE_URL: &str = "/images/TailYew.png";

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let tabs = vec![
        TabItem {
            title: "Atoms".into(),
            content: html! {
                <Typo tag={TagType::P}>{"Basic building blocks like buttons, text, and containers—designed to be reused and composed."}</Typo>
            },
        },
        TabItem {
            title: "Molecules".into(),
            content: html! {
                <Typo tag={TagType::P}>{"Small groups of elements—like forms, headers, or navigation bars—that serve a distinct purpose."}</Typo>
            },
        },
        TabItem {
            title: "Organisms".into(),
            content: html! {
                <Typo tag={TagType::P}>{"Reusable complex UI structures like cards, tables, and modal systems."}</Typo>
            },
        },
    ];

    html! {
        <div class="container mx-auto p-4">
            // Hero
            <HeroHeader
                title="TailYew: Build Beautiful Rust Frontends"
                subtitle={Some("A component system for Yew, styled with Tailwind. Build fast. Look great. Stay in Rust.".to_string())}
                background_image_url={Some(HERO_IMAGE_URL.to_string())}
            >
                <div class="flex justify-center space-x-4 mt-6">
                    <Typo tag={TagType::H2}>{ "TailYew" }</Typo>

                </div>
            </HeroHeader>

            <Spacer />

            // About
            <Section id="about" class="text-center max-w-3xl mx-auto">
                <Typo tag={TagType::H2}>{ "What is TailYew?" }</Typo>
                <Typo tag={TagType::P}>
                    { "TailYew is a reusable UI component system for the Yew framework. It uses Tailwind CSS for styling and follows Atomic Design principles. TailYew helps Rust developers build production-grade frontend UIs with zero JavaScript." }
                </Typo>
            </Section>

            <Spacer />

            // Tabs - Atomic Design
            <Section id="architecture" class="text-center">
                <Typo tag={TagType::H2}>{ "Atomic Design" }</Typo>
                <Tabs items={tabs} />
            </Section>

            <Spacer />

            // Benefits
            <Section id="benefits" class="text-center">
                <Typo tag={TagType::H2}>{ "Why TailYew?" }</Typo>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6 justify-items-center">
                    <Card id="benefit-yew" title="Yew-Powered Components" description={Some("Built with Yew 0.21, the system takes full advantage of Rust's performance and safety.")} />
                    <Card id="benefit-tailwind" title="Tailwind Styling" description={Some("Responsive, modern UIs with the full power of Tailwind, pre-configured and ready to use.")} />
                    <Card id="benefit-dx" title="Developer Experience" description={Some("Hot-reloading via `make run-frontend`, strict linting, and docs-first DX powered by a Makefile.")} />
                    <Card id="benefit-scalability" title="Atomic Design" description={Some("Organized into atoms, molecules, and organisms for scalability and reuse.")} />
                </div>
            </Section>

            <Spacer />

            // Getting Started
            <Section id="getting-started">
                <Typo tag={TagType::H2}>{ "Getting Started" }</Typo>
                <Typo tag={TagType::P}>{ "Install TailYew, add it to your Yew app, and start building UIs today." }</Typo>

                <div class="bg-gray-800 text-white p-4 rounded my-4 text-left text-sm overflow-x-auto">
                    <pre><code>{r#"cargo add tailyew"#}</code></pre>
                    <pre><code>{r#"use tailyew::atoms::Button;"#}</code></pre>
                </div>
            </Section>

            <Spacer />

            // FAQ
            <Section id="faq" class="max-w-3xl mx-auto">
                <Typo tag={TagType::H2}>{ "Frequently Asked Questions" }</Typo>
                <Accordion title="Is TailYew production-ready?">
                    <Typo tag={TagType::P}>{ "I dont know I am using it on some side projects." }</Typo>
                </Accordion>
                <Accordion title="How can I contribute?">
                    <Typo tag={TagType::P}>{ "Open issues, suggest components, or submit PRs on GitHub. We welcome community involvement!" }</Typo>
                </Accordion>
            </Section>

        </div>
    }
}
