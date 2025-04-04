// frontend/src/pages/demo_page.rs

use crate::templates::demos::{
    AComponentDemoSection, AccordionDemoSection, AppBarDemoSection, BarChartDemoSection,
    BubbleChartDemoSection, ButtonDemoSection, CardDemoSection, CheckboxDemoSection,
    CircularProgressDemoSection, ColorInputDemoSection, CopyToClipboardDemoSection,
    FileInputDemoSection, FormDemoSection, GettingStarted, HeroHeaderDemoSection,
    InitThemeDemoSection, InputDemoSection, JsonInputDemoSection, LiDemoSection,
    LineChartDemoSection, LinearProgressDemoSection, ModalButtonDemoSection, ModalDemoSection,
    NotificationDemoSection, PhoneInputDemoSection, PieChartDemoSection, PopoverDemoSection,
    RadioGroupDemoSection, RangeInputDemoSection, ScatterPlotDemoSection, SelectDemoSection,
    SpacerDemoSection, StepperDemoSection, TableDemoSection, TabsDemoSection, TextareaDemoSection,
    TooltipDemoSection, TypoDemoSection, UlDemoSection,
};
use crate::Route;
use tailyew::atoms::{Li, TagType, Typo, Ul};
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
];

#[function_component(DemoSidebar)]
pub fn demo_sidebar() -> Html {
    html! {
        <nav class="w-64 p-4 border-r border-gray-200 dark:border-gray-700 h-screen overflow-y-auto sticky top-0">
            <Typo tag={TagType::H3}>{ "Components" }</Typo>

            <Ul>
                { for DEMO_LINKS.iter().map(|link| html! {
                    <Li>
                        <Link<Route>
                            to={Route::DemoPage { component: link.route.to_string() }}
                            classes="block px-3 py-2 rounded text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 transition"
                        >
                            { link.name }
                        </Link<Route>>
                    </Li>
                }) }
            </Ul>
        </nav>
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
