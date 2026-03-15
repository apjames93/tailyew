use crate::templates::demos::DemoComponent;
use tailyew::atoms::{Avatar, Badge, Button, ButtonType, TagType, Typo, A};
use tailyew::form::{Input, InputType};
use tailyew::molecules::{
    Accordion, AvatarData, AvatarGroup, Breadcrumbs, CodeBlock, Notification, NotificationTypes,
    Tooltip, TooltipPosition,
};
use tailyew::organisms::{Card, Column, Markdown, Stepper, TabItem, Table, Tabs};
use tailyew::system::{InitTheme, Theme, ThemeOverrides};
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
struct OverrideRow {
    id: u64,
    component: String,
    slot: String,
    classes: String,
}

impl OverrideRow {
    fn new(id: u64, component: &str, slot: &str, classes: &str) -> Self {
        Self {
            id,
            component: component.to_string(),
            slot: slot.to_string(),
            classes: classes.to_string(),
        }
    }

    fn empty(id: u64) -> Self {
        Self::new(id, "", "", "")
    }

    fn from_seed(id: u64, seed: &OverrideSeed) -> Self {
        Self::new(id, seed.component, seed.slot, seed.classes)
    }
}

#[derive(Clone)]
struct OverrideSeed {
    component: &'static str,
    slot: &'static str,
    classes: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ThemePreset {
    Neon,
    Pastel,
    Ocean,
    Mono,
}

impl ThemePreset {
    fn label(self) -> &'static str {
        match self {
            Self::Neon => "NEON",
            Self::Pastel => "PASTEL",
            Self::Ocean => "Ocean",
            Self::Mono => "Mono",
        }
    }
}

struct PresetConfig {
    theme_name: &'static str,
    root_classes: &'static str,
    overrides: Vec<OverrideSeed>,
}

#[derive(Clone, Copy)]
enum OverrideField {
    Component,
    Slot,
    Classes,
}

fn classes_from_str(s: &str) -> Classes {
    s.split_whitespace()
        .map(AttrValue::from)
        .collect::<Classes>()
}

fn rows_from_seeds(seeds: &[OverrideSeed], mut next_id: u64) -> (Vec<OverrideRow>, u64) {
    let rows = seeds
        .iter()
        .map(|seed| {
            let row = OverrideRow::from_seed(next_id, seed);
            next_id += 1;
            row
        })
        .collect::<Vec<_>>();

    (rows, next_id)
}

fn preset_config(preset: ThemePreset) -> PresetConfig {
    match preset {
        ThemePreset::Neon => PresetConfig {
            theme_name: "dark",
            root_classes: "p-8 rounded-2xl space-y-6 bg-surface text-content dark:bg-surface-dark dark:text-content-invert border border-border dark:border-border-dark ring-1 ring-cyan-300/30 shadow-2xl",
            overrides: vec![
                OverrideSeed {
                    component: "A",
                    slot: "root",
                    classes: "text-cyan-300 hover:text-cyan-100 underline decoration-cyan-400/50",
                },
                OverrideSeed {
                    component: "Button",
                    slot: "root",
                    classes: "rounded-xl border border-cyan-300/40 bg-cyan-500/10 text-cyan-100 hover:bg-cyan-500/20 shadow-sm shadow-cyan-500/20",
                },
                OverrideSeed {
                    component: "Card",
                    slot: "root",
                    classes: "bg-slate-900/50 border border-slate-700 text-slate-50",
                },
                OverrideSeed {
                    component: "Badge",
                    slot: "badge",
                    classes: "bg-lime-300 text-slate-950 font-bold",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "root",
                    classes: "rounded-xl border border-slate-700 bg-slate-900/40",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "trigger",
                    classes: "text-cyan-200 hover:text-cyan-50 uppercase tracking-wide text-xs",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "content",
                    classes: "rounded-lg border border-slate-800 bg-slate-950/40 p-4",
                },
                OverrideSeed {
                    component: "Typo",
                    slot: "root",
                    classes: "text-slate-50",
                },
            ],
        },
        ThemePreset::Pastel => PresetConfig {
            theme_name: "light",
            root_classes: "p-8 rounded-3xl space-y-6 bg-surface text-content dark:bg-surface-dark dark:text-content-invert border border-border dark:border-border-dark shadow-xl",
            overrides: vec![
                OverrideSeed {
                    component: "A",
                    slot: "root",
                    classes: "text-violet-700 hover:text-fuchsia-600 underline decoration-violet-300",
                },
                OverrideSeed {
                    component: "Button",
                    slot: "root",
                    classes: "rounded-2xl bg-violet-600 text-white hover:bg-fuchsia-600 shadow-lg shadow-violet-200 font-semibold",
                },
                OverrideSeed {
                    component: "Card",
                    slot: "root",
                    classes: "bg-white border border-slate-200 shadow-lg shadow-slate-200 rounded-3xl",
                },
                OverrideSeed {
                    component: "Badge",
                    slot: "badge",
                    classes: "bg-fuchsia-500 text-white font-bold",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "root",
                    classes: "rounded-2xl border border-violet-200 bg-violet-50",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "trigger",
                    classes: "text-violet-700 font-semibold",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "content",
                    classes: "rounded-2xl border border-violet-100 bg-white p-4",
                },
                OverrideSeed {
                    component: "Typo",
                    slot: "root",
                    classes: "text-slate-900",
                },
            ],
        },
        ThemePreset::Ocean => PresetConfig {
            theme_name: "dark",
            root_classes:
                "p-8 rounded-3xl space-y-6 bg-surface text-content dark:bg-surface-dark dark:text-content-invert border-2 border-border dark:border-border-dark shadow-2xl",
            overrides: vec![
                OverrideSeed {
                    component: "A",
                    slot: "root",
                    classes: "text-emerald-300 hover:text-teal-200 underline decoration-emerald-400/60",
                },
                OverrideSeed {
                    component: "Button",
                    slot: "root",
                    classes: "rounded-2xl border-2 border-emerald-300/70 bg-teal-600 text-teal-50 hover:bg-emerald-500 font-bold shadow-md shadow-emerald-900/40",
                },
                OverrideSeed {
                    component: "Card",
                    slot: "root",
                    classes: "bg-slate-800/70 border border-teal-400/40 text-teal-50",
                },
                OverrideSeed {
                    component: "Badge",
                    slot: "badge",
                    classes: "bg-emerald-300 text-slate-900 font-black",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "root",
                    classes: "rounded-2xl border border-teal-300/60 bg-slate-800/60",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "trigger",
                    classes: "tracking-widest text-emerald-200",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "content",
                    classes: "rounded-xl border border-teal-400/30 bg-slate-900/50 p-4",
                },
                OverrideSeed {
                    component: "Typo",
                    slot: "root",
                    classes: "text-teal-50",
                },
            ],
        },
        ThemePreset::Mono => PresetConfig {
            theme_name: "light",
            root_classes: "p-8 rounded-none border-4 border-border bg-surface text-content dark:bg-surface-dark dark:text-content-invert dark:border-border-dark space-y-6",
            overrides: vec![
                OverrideSeed {
                    component: "A",
                    slot: "root",
                    classes: "text-black hover:text-gray-600 no-underline border-b border-black",
                },
                OverrideSeed {
                    component: "Button",
                    slot: "root",
                    classes: "rounded-none border-2 border-black bg-black text-white hover:bg-gray-800 uppercase tracking-widest font-bold shadow-none",
                },
                OverrideSeed {
                    component: "Card",
                    slot: "root",
                    classes: "rounded-none border-4 border-black bg-white shadow-none",
                },
                OverrideSeed {
                    component: "Card",
                    slot: "body",
                    classes: "rounded-none",
                },
                OverrideSeed {
                    component: "Badge",
                    slot: "badge",
                    classes: "rounded-none bg-black text-white border border-white",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "root",
                    classes: "rounded-none border-4 border-black bg-gray-100",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "trigger",
                    classes: "rounded-none uppercase tracking-widest text-black",
                },
                OverrideSeed {
                    component: "Tabs",
                    slot: "content",
                    classes: "rounded-none border-2 border-black bg-white p-4",
                },
                OverrideSeed {
                    component: "Typo",
                    slot: "root",
                    classes: "uppercase tracking-wide",
                },
            ],
        },
    }
}

fn apply_preset(
    preset: ThemePreset,
    theme_name: &UseStateHandle<String>,
    root_classes: &UseStateHandle<String>,
    overrides: &UseStateHandle<Vec<OverrideRow>>,
    active_preset: &UseStateHandle<ThemePreset>,
    next_row_id: &UseStateHandle<u64>,
    editor_epoch: &UseStateHandle<u64>,
) {
    let preset_data = preset_config(preset);
    let (rows, next_id) = rows_from_seeds(&preset_data.overrides, **next_row_id);

    theme_name.set(preset_data.theme_name.to_string());
    root_classes.set(preset_data.root_classes.to_string());
    overrides.set(rows);
    active_preset.set(preset);
    next_row_id.set(next_id);
    editor_epoch.set(**editor_epoch + 1);
}

fn update_override_field(
    rows: &UseStateHandle<Vec<OverrideRow>>,
    row_id: u64,
    field: OverrideField,
    value: String,
) {
    let mut next_rows = (**rows).clone();
    if let Some(row) = next_rows.iter_mut().find(|row| row.id == row_id) {
        match field {
            OverrideField::Component => row.component = value,
            OverrideField::Slot => row.slot = value,
            OverrideField::Classes => row.classes = value,
        }
    }
    rows.set(next_rows);
}

fn build_theme_overrides(rows: &[OverrideRow]) -> ThemeOverrides {
    let mut built = ThemeOverrides::new();

    for row in rows {
        let component = row.component.trim();
        let slot = row.slot.trim();

        if component.is_empty() || slot.is_empty() {
            continue;
        }

        built.insert(
            component.to_string(),
            slot.to_string(),
            classes_from_str(row.classes.as_str()),
        );
    }

    built
}

#[component(InitThemeDemoSection)]
pub fn init_theme_demo_section() -> Html {
    let initial_preset = ThemePreset::Neon;
    let initial = preset_config(initial_preset);
    let (initial_rows, initial_next_id) = rows_from_seeds(&initial.overrides, 1);

    let theme_name = use_state(|| initial.theme_name.to_string());
    let root_classes = use_state(|| initial.root_classes.to_string());
    let overrides = use_state(|| initial_rows);
    let active_preset = use_state(|| initial_preset);
    let next_row_id = use_state(|| initial_next_id);
    let editor_epoch = use_state(|| 0_u64);

    let on_theme_name_change = {
        let theme_name = theme_name.clone();
        Callback::from(move |val: String| theme_name.set(val))
    };

    let on_root_classes_change = {
        let root_classes = root_classes.clone();
        Callback::from(move |val: String| root_classes.set(val))
    };

    let on_add_override = {
        let overrides = overrides.clone();
        let next_row_id = next_row_id.clone();
        Callback::from(move |_| {
            let id = *next_row_id;
            next_row_id.set(id + 1);

            let mut next_rows = (*overrides).clone();
            next_rows.push(OverrideRow::empty(id));
            overrides.set(next_rows);
        })
    };

    let theme = Theme {
        name: (*theme_name).clone(),
        class: classes_from_str(root_classes.as_str()),
        overrides: build_theme_overrides(overrides.as_ref()),
    };

    let table_columns = vec![
        Column {
            header: html! { "Package" },
            values: vec![html! { "@tailyew/ui" }, html! { "@tailyew/charts" }],
        },
        Column {
            header: html! { "Status" },
            values: vec![html! { "Stable" }, html! { "Beta" }],
        },
        Column {
            header: html! { "Themeable" },
            values: vec![html! { "Yes" }, html! { "Yes" }],
        },
    ];

    let tabs_items = vec![
        TabItem {
            title: "Overview".into(),
            content: html! {
                <Typo>{"Use overrides to make this section look like your design system."}</Typo>
            },
        },
        TabItem {
            title: "Buttons".into(),
            content: html! {
                <div class="flex flex-wrap gap-2">
                    <Button button_type={ButtonType::Primary}>{"Primary"}</Button>
                    <Button button_type={ButtonType::Secondary}>{"Secondary"}</Button>
                    <Button button_type={ButtonType::Danger}>{"Danger"}</Button>
                </div>
            },
        },
        TabItem {
            title: "Code".into(),
            content: html! {
                <CodeBlock language={Some("rust".to_string())} show_copy={false}>
                    {"let themed = use_themed_classes(\"Button\", \"root\", defaults, class);"}
                </CodeBlock>
            },
        },
    ];

    let avatar_group_data = vec![
        AvatarData {
            fallback: Some("AJ".into()),
            alt: Some("Alex James".into()),
            ..AvatarData::default()
        },
        AvatarData {
            fallback: Some("TS".into()),
            alt: Some("Taylor Stone".into()),
            ..AvatarData::default()
        },
        AvatarData {
            fallback: Some("MK".into()),
            alt: Some("Morgan Kim".into()),
            ..AvatarData::default()
        },
    ];

    let markdown_preview = "## Markdown Preview\n\n- Theme tokens can map to Tailwind utilities.\n- Overrides are keyed by component and slot.\n- Live editing helps teams iterate quickly.\n\nVisit [TailYew](https://github.com/apjames93/tailyew).";

    let theme_btns = html! {
        <Card
            title={"Preset Themes".to_string()}
            subtitle={Some("Each preset overwrites theme name, root classes, and override rows.".to_string())}
        >
            <div class="flex flex-wrap gap-2">
                {
                    for [ThemePreset::Neon, ThemePreset::Pastel, ThemePreset::Ocean, ThemePreset::Mono]
                        .into_iter()
                        .map(|preset| {
                            let theme_name = theme_name.clone();
                            let root_classes = root_classes.clone();
                            let overrides = overrides.clone();
                            let active_preset = active_preset.clone();
                            let next_row_id = next_row_id.clone();
                            let editor_epoch = editor_epoch.clone();

                            let is_active = *active_preset == preset;
                            let button_type = if is_active {
                                ButtonType::Primary
                            } else {
                                ButtonType::Ghost
                            };
                            let preset_class = match preset {
                                ThemePreset::Neon => "border border-cyan-400/60 text-cyan-300",
                                ThemePreset::Pastel => "border border-fuchsia-300 text-violet-700",
                                ThemePreset::Ocean => "border-2 border-teal-400/70 text-emerald-300",
                                ThemePreset::Mono => "border-2 border-black text-black rounded-none",
                            };

                            let onclick = Callback::from(move |_| {
                                apply_preset(
                                    preset,
                                    &theme_name,
                                    &root_classes,
                                    &overrides,
                                    &active_preset,
                                    &next_row_id,
                                    &editor_epoch,
                                );
                            });

                            html! {
                                <Button
                                    button_type={button_type}
                                    on_click={onclick}
                                    class={classes!("text-xs", "px-3", "py-1", preset_class, if is_active { "ring-2 ring-offset-1" } else { "" })}
                                >
                                    { html! { { preset.label() } } }
                                </Button>
                            }
                        })
                }
            </div>
        </Card>
    };

    let editor_content = html! {
        <div class="space-y-4 text-left">
            <Card
                title={"Theme Root".to_string()}
                subtitle={Some("Set `Theme { name, class }` values used by InitTheme.".to_string())}
            >
                <Input
                    key={format!("theme-name-{}", *editor_epoch)}
                    id="theme-playground-theme-name"
                    label="Theme Name (data-theme)"
                    placeholder="dark"
                    input_type={InputType::Text}
                    default_value={(*theme_name).clone()}
                    on_change={Some(on_theme_name_change)}
                />

                <Input
                    key={format!("theme-root-classes-{}", *editor_epoch)}
                    id="theme-playground-root-classes"
                    label="Root Classes"
                    placeholder="p-6 rounded-xl bg-surface text-content dark:bg-surface-dark dark:text-content-invert"
                    input_type={InputType::Text}
                    default_value={(*root_classes).clone()}
                    on_change={Some(on_root_classes_change)}
                />
            </Card>

            <Card
                title={"Override Rows".to_string()}
                subtitle={Some("Component + slot + class string. Empty component/slot rows are ignored.".to_string())}
            >
                <div class="mb-3 flex justify-end">
                    <Button button_type={ButtonType::Secondary} on_click={on_add_override} class="text-xs px-2 py-1">
                        {"Add Row"}
                    </Button>
                </div>

                <div class="space-y-3">
                    {
                        for overrides.iter().map(|row| {
                            let row_id = row.id;

                            let on_component_change = {
                                let overrides = overrides.clone();
                                Callback::from(move |value: String| {
                                    update_override_field(
                                        &overrides,
                                        row_id,
                                        OverrideField::Component,
                                        value,
                                    );
                                })
                            };

                            let on_slot_change = {
                                let overrides = overrides.clone();
                                Callback::from(move |value: String| {
                                    update_override_field(
                                        &overrides,
                                        row_id,
                                        OverrideField::Slot,
                                        value,
                                    );
                                })
                            };

                            let on_classes_change = {
                                let overrides = overrides.clone();
                                Callback::from(move |value: String| {
                                    update_override_field(
                                        &overrides,
                                        row_id,
                                        OverrideField::Classes,
                                        value,
                                    );
                                })
                            };

                            let on_remove = {
                                let overrides = overrides.clone();
                                Callback::from(move |_| {
                                    let next_rows = overrides
                                        .iter()
                                        .filter(|row| row.id != row_id)
                                        .cloned()
                                        .collect::<Vec<_>>();
                                    overrides.set(next_rows);
                                })
                            };

                            html! {
                                <div key={row.id.to_string()} class="rounded-lg border border-gray-200 p-3 dark:border-gray-700">
                                    <div class="grid gap-2 lg:grid-cols-12">
                                        <div class="lg:col-span-3">
                                            <Input
                                                key={format!("row-{}-component", row.id)}
                                                id={format!("override-component-{}", row.id)}
                                                label="Component"
                                                placeholder="Button"
                                                input_type={InputType::Text}
                                                default_value={row.component.clone()}
                                                on_change={Some(on_component_change)}
                                            />
                                        </div>

                                        <div class="lg:col-span-2">
                                            <Input
                                                key={format!("row-{}-slot", row.id)}
                                                id={format!("override-slot-{}", row.id)}
                                                label="Slot"
                                                placeholder="root"
                                                input_type={InputType::Text}
                                                default_value={row.slot.clone()}
                                                on_change={Some(on_slot_change)}
                                            />
                                        </div>

                                        <div class="lg:col-span-6">
                                            <Input
                                                key={format!("row-{}-classes", row.id)}
                                                id={format!("override-classes-{}", row.id)}
                                                label="Classes"
                                                placeholder="rounded-xl border border-cyan-400"
                                                input_type={InputType::Text}
                                                default_value={row.classes.clone()}
                                                on_change={Some(on_classes_change)}
                                            />
                                        </div>

                                        <div class="flex items-end lg:col-span-1">
                                            <Button
                                                button_type={ButtonType::Danger}
                                                class="mb-4 w-full"
                                                on_click={on_remove}
                                            >
                                                {"Remove"}
                                            </Button>
                                        </div>
                                    </div>
                                </div>
                            }
                        })
                    }
                </div>
            </Card>

            <Card
                title={"Common Override Keys".to_string()}
                subtitle={Some("Slot names are component-defined. Use these as quick starters.".to_string())}
            >
                <CodeBlock show_copy={false}>
                    {"A / root\nButton / root\nCard / root\nCard / body\nCard / image\nBadge / root\nBadge / badge\nTabs / root\nTabs / trigger\nTabs / content\nAccordion / trigger"}
                </CodeBlock>
            </Card>
        </div>
    };

    let example = html! {
        <div class="grid gap-6 xl:grid-cols-[minmax(320px,460px)_minmax(0,1fr)] text-left">
            <div class="space-y-4">
                {theme_btns}
                <Accordion
                    title={html! {
                        <div class="text-left">
                            <Typo tag={TagType::H4}>{"Theme Playground Controls"}</Typo>
                        </div>
                    }}
                    default_open={false}
                    class="border border-gray-300 dark:border-gray-700"
                    content_class="space-y-2"
                >
                    { editor_content }
                </Accordion>
            </div>

            <div class="space-y-4">
                <Typo tag={TagType::H4}>{"Live Preview"}</Typo>

                <InitTheme theme={Some(theme)}>
                    <div
                        class="space-y-6 rounded-lg border border-gray-300/50 p-4 text-left"
                        data-theme-debug={format!(
                            "preset={};theme={};overrides={}",
                            (*active_preset).label(),
                            (*theme_name).as_str(),
                            overrides.len()
                        )}
                    >
                        <div class="flex flex-wrap items-center justify-between gap-3">
                            <Typo tag={TagType::H3}>{"TailYew Theme Playground"}</Typo>
                            <A
                                href="https://github.com/apjames93/tailyew"
                                target={Some(AttrValue::from("_blank"))}
                            >
                                {"View source"}
                            </A>
                        </div>

                        <div class="flex flex-wrap gap-2">
                            <Button button_type={ButtonType::Primary}>{"Primary"}</Button>
                            <Button button_type={ButtonType::Secondary}>{"Secondary"}</Button>
                            <Button button_type={ButtonType::Ghost}>{"Ghost"}</Button>
                            <Badge badge_content={Some("7".to_string())}>
                                <Button button_type={ButtonType::Button}>{"Inbox"}</Button>
                            </Badge>
                        </div>

                        <div class="grid gap-4 lg:grid-cols-2">
                            <Card
                                title={"Themed Card"}
                                subtitle={Some("Slot override demo".to_string())}
                                description={Some("Try Card/root and Card/body in the editor.".to_string())}
                            >
                                <A href="#">{"Card action link"}</A>
                            </Card>

                            <div class="space-y-4">
                                <Accordion title={html! { "Accordion" }} default_open={true}>
                                    <Typo>{"Accordion supports root, trigger, and content slots."}</Typo>
                                </Accordion>

                                <Notification
                                    message={"Preview notification".to_string()}
                                    notification_type={NotificationTypes::Info}
                                    visible={true}
                                    show_close={false}
                                    fixed={false}
                                    max_width={"max-w-full".to_string()}
                                />
                            </div>
                        </div>

                        <Breadcrumbs>
                            <A href="#">{"Home"}</A>
                            <A href="#">{"System"}</A>
                            <Typo>{"Theme Playground"}</Typo>
                        </Breadcrumbs>

                        <div class="flex flex-wrap items-center gap-4">
                            <Avatar
                                fallback={Some(AttrValue::from("TY"))}
                                alt={Some(AttrValue::from("TailYew"))}
                            />
                            <AvatarGroup avatars={avatar_group_data} max_visible={5} />
                            <Tooltip
                                trigger={html! { <Button button_type={ButtonType::Ghost}>{"Hover me"}</Button> }}
                                content={html! { "Tooltip content" }}
                                position={TooltipPosition::Top}
                            />
                        </div>

                        <Tabs
                            items={tabs_items}
                            scroll_into_view={false}
                            id_prefix={Some(AttrValue::from("theme-playground-tabs"))}
                        />

                        <Table columns={table_columns} />

                        <Stepper
                            steps={vec![
                                "Design tokens".to_string(),
                                "Component slots".to_string(),
                                "Ship theme".to_string(),
                            ]}
                            show_navigation_buttons={true}
                        />

                        <Markdown content={markdown_preview.to_string()} />
                    </div>
                </InitTheme>
            </div>
        </div>
    };

    let usage_code = r#"
let theme = Theme {
    name: "dark".into(),
    class: classes!(
        "p-6",
        "rounded-xl",
        "bg-surface",
        "text-content",
        "dark:bg-surface-dark",
        "dark:text-content-invert"
    ),
    overrides: ThemeOverrides::new()
        .set("A", "root", classes!("text-cyan-300", "hover:text-cyan-100"))
        .set("Button", "root", classes!("rounded-xl"))
        .set("Card", "root", classes!("border", "border-cyan-700"))
        .set("Tabs", "trigger", classes!("uppercase", "tracking-wide", "text-xs")),
};

// Slot names are component-defined. Check each component's docs/source.
html! {
    <InitTheme theme={Some(theme)}>
        { "Your themed UI" }
    </InitTheme>
};
"#;

    let props_table = vec![
        Column {
            header: html! { "Prop" },
            values: vec![html! { "theme" }, html! { "class" }],
        },
        Column {
            header: html! { "Type" },
            values: vec![html! { "Option<Theme>" }, html! { "Classes" }],
        },
        Column {
            header: html! { "Description" },
            values: vec![
                html! { "Theme config with name, root class, and component/slot overrides." },
                html! { "Extra classes merged on InitTheme's root container." },
            ],
        },
    ];

    html! {
        <DemoComponent
            github_demo_path="system/init_theme_demo_section.rs"
            github_source_path="system/theme.rs"
            title="InitTheme Live Playground"
            description={Some(html! {
                <p>{"Live editor for `Theme { name, class, overrides }` with preset switching and slot-level Tailwind overrides across many TailYew components."}</p>
            })}
            example={example}
            usage_code={usage_code}
            props_table={Some(props_table)}
        />
    }
}
