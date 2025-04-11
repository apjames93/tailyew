// frontend/src/pages/demo_page.rs

use crate::templates::demos::{
    AComponentDemoSection, AccordionDemoSection, AppBarDemoSection, BarChartDemoSection,
    BubbleChartDemoSection, ButtonDemoSection, CardDemoSection, CheckboxDemoSection,
    CircularProgressDemoSection, CodeBlockDemoSection, ColorInputDemoSection,
    CopyToClipboardDemoSection, DownloadButtonDemoSection, FileInputDemoSection, FormDemoSection,
    GettingStarted, HeroHeaderDemoSection, ImageDemoSection, InitThemeDemoSection,
    InputDemoSection, JsonInputDemoSection, LiDemoSection, LineChartDemoSection,
    LinearProgressDemoSection, MarkdownDemoSection, ModalButtonDemoSection, ModalDemoSection,
    NavBarDemoSection, NestedListDemoSection, NotificationDemoSection, PhoneInputDemoSection,
    PieChartDemoSection, PopoverDemoSection, RadioGroupDemoSection, RangeInputDemoSection,
    ScatterPlotDemoSection, SelectDemoSection, SidebarDemoSection, SpacerDemoSection,
    StepperDemoSection, TableDemoSection, TabsDemoSection, TextareaDemoSection, TooltipDemoSection,
    TypoDemoSection, UlDemoSection,
};
use crate::Route;
use std::collections::HashMap;
use tailyew::organisms::{NestedItem, Sidebar, SidebarButton};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Clone)]
struct DemoLink {
    name: &'static str,
    route: &'static str,
    render: fn() -> Html,
    ctype: &'static str,
}

const DEMO_LINKS: &[DemoLink] = &[
    // System-level or getting started
    DemoLink {
        ctype: "system",
        name: "Getting Started",
        route: "getting_started",
        render: || html! { <GettingStarted /> },
    },
    // Atoms (basic UI elements)
    DemoLink {
        ctype: "atoms",
        name: "A",
        route: "a",
        render: || html! { <AComponentDemoSection /> },
    },
    DemoLink {
        ctype: "atoms",
        name: "Button",
        route: "button",
        render: || html! { <ButtonDemoSection /> },
    },
    DemoLink {
        ctype: "atoms",
        name: "Typo",
        route: "typo",
        render: || html! { <TypoDemoSection /> },
    },
    DemoLink {
        ctype: "atoms",
        name: "Li",
        route: "li",
        render: || html! { <LiDemoSection /> },
    },
    DemoLink {
        ctype: "atoms",
        name: "Ul",
        route: "ul",
        render: || html! { <UlDemoSection /> },
    },
    DemoLink {
        ctype: "atoms",
        name: "Spacer",
        route: "spacer",
        render: || html! { <SpacerDemoSection /> },
    },
    DemoLink {
        ctype: "atoms",
        name: "Image",
        route: "image",
        render: || html! { <ImageDemoSection /> },
    },
    // Molecules (interactive components composed of atoms)
    DemoLink {
        ctype: "molecules",
        name: "Accordion",
        route: "accordion",
        render: || html! { <AccordionDemoSection /> },
    },
    DemoLink {
        ctype: "molecules",
        name: "Modal Button",
        route: "modal_button",
        render: || html! { <ModalButtonDemoSection /> },
    },
    DemoLink {
        ctype: "molecules",
        name: "Copy to Clipboard",
        route: "copy_to_clipboard",
        render: || html! { <CopyToClipboardDemoSection /> },
    },
    DemoLink {
        ctype: "molecules",
        name: "Download Button",
        route: "download_button",
        render: || html! { <DownloadButtonDemoSection /> },
    },
    DemoLink {
        ctype: "molecules",
        name: "Tooltip",
        route: "tooltip",
        render: || html! { <TooltipDemoSection /> },
    },
    DemoLink {
        ctype: "molecules",
        name: "Popover",
        route: "popover",
        render: || html! { <PopoverDemoSection /> },
    },
    // Organisms (large composite structures)
    DemoLink {
        ctype: "organisms",
        name: "Card",
        route: "card",
        render: || html! { <CardDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "Table",
        route: "table",
        render: || html! { <TableDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "Nested List",
        route: "nested_list",
        render: || html! { <NestedListDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "NavBar",
        route: "navbar",
        render: || html! { <NavBarDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "App Bar",
        route: "app_bar",
        render: || html! { <AppBarDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "Notification",
        route: "notification",
        render: || html! { <NotificationDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "Stepper",
        route: "stepper",
        render: || html! { <StepperDemoSection /> },
    },
    // Forms
    DemoLink {
        ctype: "forms",
        name: "Form",
        route: "form",
        render: || html! { <FormDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Input",
        route: "input",
        render: || html! { <InputDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Select",
        route: "select",
        render: || html! { <SelectDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Textarea",
        route: "textarea",
        render: || html! { <TextareaDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Checkbox",
        route: "checkbox",
        render: || html! { <CheckboxDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Radio Group",
        route: "radio_group",
        render: || html! { <RadioGroupDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Color Input",
        route: "color_input",
        render: || html! { <ColorInputDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Phone Input",
        route: "phone_input",
        render: || html! { <PhoneInputDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "Range Input",
        route: "range_input",
        render: || html! { <RangeInputDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "File Input",
        route: "file_input",
        render: || html! { <FileInputDemoSection /> },
    },
    DemoLink {
        ctype: "forms",
        name: "JSON Input",
        route: "json_input",
        render: || html! { <JsonInputDemoSection /> },
    },
    // Charts
    DemoLink {
        ctype: "charts",
        name: "Bar Chart",
        route: "bar_chart",
        render: || html! { <BarChartDemoSection /> },
    },
    DemoLink {
        ctype: "charts",
        name: "Bubble Chart",
        route: "bubble_chart",
        render: || html! { <BubbleChartDemoSection /> },
    },
    DemoLink {
        ctype: "charts",
        name: "Line Chart",
        route: "line_chart",
        render: || html! { <LineChartDemoSection /> },
    },
    DemoLink {
        ctype: "charts",
        name: "Pie Chart",
        route: "pie_chart",
        render: || html! { <PieChartDemoSection /> },
    },
    DemoLink {
        ctype: "charts",
        name: "Scatter Chart",
        route: "scatter_chart",
        render: || html! { <ScatterPlotDemoSection /> },
    },
    DemoLink {
        ctype: "charts",
        name: "Linear Progress",
        route: "linear_progress",
        render: || html! { <LinearProgressDemoSection /> },
    },
    DemoLink {
        ctype: "charts",
        name: "Circular Progress",
        route: "circular_progress",
        render: || html! { <CircularProgressDemoSection /> },
    },
    // Utilities / System
    DemoLink {
        ctype: "system",
        name: "Init Theme",
        route: "init_theme",
        render: || html! { <InitThemeDemoSection /> },
    },
    DemoLink {
        ctype: "system",
        name: "Hero Header",
        route: "hero_header",
        render: || html! { <HeroHeaderDemoSection /> },
    },
    DemoLink {
        ctype: "system",
        name: "Tabs",
        route: "tabs",
        render: || html! { <TabsDemoSection /> },
    },
    DemoLink {
        ctype: "system",
        name: "Modal",
        route: "modal",
        render: || html! { <ModalDemoSection /> },
    },
    DemoLink {
        ctype: "system",
        name: "CodeBlock",
        route: "code_block",
        render: || html! { <CodeBlockDemoSection /> },
    },
    DemoLink {
        ctype: "system",
        name: "Markdown",
        route: "markdown",
        render: || html! { <MarkdownDemoSection /> },
    },
    DemoLink {
        ctype: "organisms",
        name: "Sidebar",
        route: "sidebar",
        render: || html! { <SidebarDemoSection /> },
    },
];

fn build_nested_sidebar_links() -> Vec<NestedItem> {
    // Group DemoLinks by component type
    let mut grouped: HashMap<&str, Vec<NestedItem>> = HashMap::new();

    for link in DEMO_LINKS {
        let item = NestedItem::with_value(link.name, link.route);
        grouped.entry(link.ctype).or_default().push(item);
    }

    // Define the order of groups
    let group_order = vec![
        "system",
        "atoms",
        "molecules",
        "organisms",
        "charts",
        "forms",
    ];

    // Compose NestedItems grouped and ordered
    let mut sidebar_items = Vec::new();
    for group in group_order {
        if let Some(children) = grouped.remove(group) {
            // Capitalize group label
            let title = match group {
                "system" => "System",
                "atoms" => "Atoms",
                "molecules" => "Molecules",
                "organisms" => "Organisms",
                "charts" => "Charts",
                "forms" => "Forms",
                _ => group,
            };
            sidebar_items.push(NestedItem::with_children(title, children));
        }
    }

    sidebar_items
}

#[function_component(DemoSidebar)]
pub fn demo_sidebar() -> Html {
    let navigator = use_navigator();

    let on_select = {
        let Some(navigator) = navigator else {
            return html! { <div>{"Navigator not available"}</div> };
        };

        Callback::from(move |value: AttrValue| {
            navigator.push(&Route::DemoPage {
                component: value.to_string(),
            });
        })
    };

    html! {
        <Sidebar
            on_select={on_select}
            top_offset_class={classes!("top-16")}
            icon_list={vec![
                SidebarButton {
                    icon: html! {
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    },
                    list: build_nested_sidebar_links(),
                }
            ]}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct DemoPageProps {
    pub component: String,
}

#[function_component(DemoPage)]
pub fn demo_page(props: &DemoPageProps) -> Html {
    let DemoPageProps { component } = props;

    let content = DEMO_LINKS
        .iter()
        .find(|link| link.route.eq_ignore_ascii_case(component))
        .map(|link| (link.render)())
        .unwrap_or_else(|| {
            html! {
                <div class="text-center mt-16 text-red-500">
                    { format!("No demo found for component: {}", component) }
                </div>
            }
        });

    html! {
        <div class="flex">
            <DemoSidebar />
            <div class="flex-1 p-6 overflow-auto">
                { content }
            </div>
        </div>
    }
}
