// frontend/src/pages/demo_page.rs

use crate::templates::demos::{
    AComponentDemoSection, AccordionDemoSection, AppBarDemoSection, BarChartDemoSection,
    BubbleChartDemoSection, ButtonDemoSection, CardDemoSection, CheckboxDemoSection,
    CircularProgressDemoSection, CodeBlockDemoSection, ColorInputDemoSection,
    CopyToClipboardDemoSection, FileInputDemoSection, FormDemoSection, GettingStarted,
    HeroHeaderDemoSection, ImageDemoSection, InitThemeDemoSection, InputDemoSection,
    JsonInputDemoSection, LiDemoSection, LineChartDemoSection, LinearProgressDemoSection,
    MarkdownDemoSection, ModalButtonDemoSection, ModalDemoSection, NavBarDemoSection,
    NotificationDemoSection, PhoneInputDemoSection, PieChartDemoSection, PopoverDemoSection,
    RadioGroupDemoSection, RangeInputDemoSection, ScatterPlotDemoSection, SelectDemoSection,
    SpacerDemoSection, StepperDemoSection, TableDemoSection, TabsDemoSection, TextareaDemoSection,
    TooltipDemoSection, TypoDemoSection, UlDemoSection,
};
use crate::Route;
use tailyew::atoms::{Button, ButtonType, Li, TagType, Typo, Ul};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone)]
struct DemoLink {
    name: &'static str,
    route: &'static str,
    render: fn() -> Html,
}

const DEMO_LINKS: &[DemoLink] = &[
    DemoLink {
        name: "Accordion",
        route: "accordion",
        render: || html! { <AccordionDemoSection /> },
    },
    DemoLink {
        name: "App Bar",
        route: "app_bar",
        render: || html! { <AppBarDemoSection /> },
    },
    DemoLink {
        name: "Button",
        route: "button",
        render: || html! { <ButtonDemoSection /> },
    },
    DemoLink {
        name: "Card",
        route: "card",
        render: || html! { <CardDemoSection /> },
    },
    DemoLink {
        name: "Checkbox",
        route: "checkbox",
        render: || html! { <CheckboxDemoSection /> },
    },
    DemoLink {
        name: "Color Input",
        route: "color_input",
        render: || html! { <ColorInputDemoSection /> },
    },
    DemoLink {
        name: "Copy to Clipboard",
        route: "copy_to_clipboard",
        render: || html! { <CopyToClipboardDemoSection /> },
    },
    DemoLink {
        name: "File Input",
        route: "file_input",
        render: || html! { <FileInputDemoSection /> },
    },
    DemoLink {
        name: "Form",
        route: "form",
        render: || html! { <FormDemoSection /> },
    },
    DemoLink {
        name: "Hero Header",
        route: "hero_header",
        render: || html! { <HeroHeaderDemoSection /> },
    },
    DemoLink {
        name: "Input",
        route: "input",
        render: || html! { <InputDemoSection /> },
    },
    DemoLink {
        name: "JSON Input",
        route: "json_input",
        render: || html! { <JsonInputDemoSection /> },
    },
    DemoLink {
        name: "Line Chart",
        route: "line_chart",
        render: || html! { <LineChartDemoSection /> },
    },
    DemoLink {
        name: "Linear Progress",
        route: "linear_progress",
        render: || html! { <LinearProgressDemoSection /> },
    },
    DemoLink {
        name: "Modal",
        route: "modal",
        render: || html! { <ModalDemoSection /> },
    },
    DemoLink {
        name: "Modal Button",
        route: "modal_button",
        render: || html! { <ModalButtonDemoSection /> },
    },
    DemoLink {
        name: "Notification",
        route: "notification",
        render: || html! { <NotificationDemoSection /> },
    },
    DemoLink {
        name: "Phone Input",
        route: "phone_input",
        render: || html! { <PhoneInputDemoSection /> },
    },
    DemoLink {
        name: "Popover",
        route: "popover",
        render: || html! { <PopoverDemoSection /> },
    },
    DemoLink {
        name: "Radio Group",
        route: "radio_group",
        render: || html! { <RadioGroupDemoSection /> },
    },
    DemoLink {
        name: "Range Input",
        route: "range_input",
        render: || html! { <RangeInputDemoSection /> },
    },
    DemoLink {
        name: "Scatter Chart",
        route: "scatter_chart",
        render: || html! { <ScatterPlotDemoSection /> },
    },
    DemoLink {
        name: "Select",
        route: "select",
        render: || html! { <SelectDemoSection /> },
    },
    DemoLink {
        name: "Spacer",
        route: "spacer",
        render: || html! { <SpacerDemoSection /> },
    },
    DemoLink {
        name: "Stepper",
        route: "stepper",
        render: || html! { <StepperDemoSection /> },
    },
    DemoLink {
        name: "Table",
        route: "table",
        render: || html! { <TableDemoSection /> },
    },
    DemoLink {
        name: "Tabs",
        route: "tabs",
        render: || html! { <TabsDemoSection /> },
    },
    DemoLink {
        name: "Textarea",
        route: "textarea",
        render: || html! { <TextareaDemoSection /> },
    },
    DemoLink {
        name: "Tooltip",
        route: "tooltip",
        render: || html! { <TooltipDemoSection /> },
    },
    DemoLink {
        name: "Typo",
        route: "typo",
        render: || html! { <TypoDemoSection /> },
    },
    DemoLink {
        name: "A",
        route: "a",
        render: || html! { <AComponentDemoSection /> },
    },
    DemoLink {
        name: "Circular Progress",
        route: "circular_progress",
        render: || html! { <CircularProgressDemoSection /> },
    },
    DemoLink {
        name: "Li",
        route: "li",
        render: || html! { <LiDemoSection /> },
    },
    DemoLink {
        name: "Ul",
        route: "ul",
        render: || html! { <UlDemoSection /> },
    },
    DemoLink {
        name: "Bar Chart",
        route: "bar_chart",
        render: || html! { <BarChartDemoSection /> },
    },
    DemoLink {
        name: "Bubble Chart",
        route: "bubble_chart",
        render: || html! { <BubbleChartDemoSection /> },
    },
    DemoLink {
        name: "Pie Chart",
        route: "pie_chart",
        render: || html! { <PieChartDemoSection /> },
    },
    DemoLink {
        name: "Init Theme",
        route: "init_theme",
        render: || html! { <InitThemeDemoSection /> },
    },
    DemoLink {
        name: "Getting Started",
        route: "getting_started",
        render: || html! { <GettingStarted /> },
    },
    DemoLink {
        name: "Navbar",
        route: "navbar",
        render: || html! { <NavBarDemoSection /> },
    },
    DemoLink {
        name: "Markdown",
        route: "markdown",
        render: || html! { <MarkdownDemoSection /> },
    },
    DemoLink {
        name: "CodeBlock",
        route: "code_block",
        render: || html! { <CodeBlockDemoSection /> },
    },
    DemoLink {
        name: "Image",
        route: "image",
        render: || html! { <ImageDemoSection /> },
    },
];

#[function_component(DemoSidebar)]
pub fn demo_sidebar() -> Html {
    let is_open = use_state(|| true);

    let toggle_sidebar = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    let close_sidebar = {
        let is_open = is_open.clone();
        Callback::from(move |_: MouseEvent| is_open.set(false))
    };

    html! {
        <>
            // Sidebar Toggle Column (always visible, narrow)
            <div class="w-12 md:w-16 p-2 pt-4 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700 sticky top-0 h-screen z-20 flex flex-col items-center">
                <Button
                    button_type={ButtonType::Ghost}
                    onclick={toggle_sidebar}
                    class="p-2 rounded-md"
                >
                    <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"/>
                    </svg>
                </Button>
            </div>

            // Sidebar Drawer (conditional)
            {
                if *is_open {
                    html! {
                        <div class="w-64 bg-white dark:bg-gray-900 h-screen border-r border-gray-200 dark:border-gray-700 p-4 overflow-y-auto sticky top-0 z-10">
                            <Typo tag={TagType::H3} class="mb-4">{ "Components" }</Typo>
                            <Ul>
                                { for DEMO_LINKS.iter().map(|link| html! {
                                    <Li onclick={close_sidebar.clone()}>
                                        <Link<Route>
                                            to={Route::DemoPage { component: link.route.to_string() }}
                                            classes="block px-3 py-2 rounded text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition"

                                        >
                                            { link.name }
                                        </Link<Route>>
                                    </Li>
                                }) }
                            </Ul>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
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
