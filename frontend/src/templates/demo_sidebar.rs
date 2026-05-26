// frontend/src/pages/demo_page.rs

use crate::Route;
use crate::templates::demos::{
    AComponentDemoSection, AccordionDemoSection, AppBarDemoSection, AvatarDemoSection,
    AvatarGroupDemoSection, BadgeDemoSection, BarChartDemoSection, BreadcrumbsDemoSection,
    BubbleChartDemoSection, ButtonDemoSection, CardDemoSection, CheckboxDemoSection,
    ChipDemoSection, CircularProgressDemoSection, CodeBlockDemoSection, ColorInputDemoSection,
    CopyToClipboardDemoSection, DownloadButtonDemoSection, FieldArrayDemoSection,
    FileInputDemoSection, FormBuilderDemoSection, FormDemoSection, FormModalDemoSection,
    GettingStarted, HeroHeaderDemoSection, IconDemoSection, ImageDemoSection, InitThemeDemoSection,
    InputDemoSection, JsonBackedFormDemoSection, JsonInputDemoSection, KeyValueInputDemoSection,
    LabelDemoSection, LiDemoSection, LineChartDemoSection, LinearProgressDemoSection,
    MarkdownDemoSection, ModalButtonDemoSection, ModalDemoSection, NavBarDemoSection,
    NestedListDemoSection, NotificationDemoSection, PhoneInputDemoSection, PieChartDemoSection,
    PopoverDemoSection, RadioGroupDemoSection, RangeInputDemoSection, ScatterPlotDemoSection,
    SearchInputDemoSection, SelectDemoSection, SidebarDemoSection, SpacerDemoSection,
    StaticValuesInputDemoSection, StepperDemoSection, SwitchDemoSection, TableDemoSection,
    TabsDemoSection, TagsInputDemoSection, TextareaDemoSection, TooltipDemoSection,
    TypoDemoSection, UlDemoSection, VideoDemoSection,
};
use tailyew::{
    AppsIcon, AtomIcon, BarChartIcon, FormIcon, NestedItem, PolylineIcon, Sidebar, SidebarButton,
    SystemIcon,
};
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Clone)]
pub struct DemoLink {
    pub name: &'static str,
    pub route: &'static str,
    pub render: fn() -> Html,
}

pub const SYSTEM_LINKS: &[DemoLink] = &[
    DemoLink {
        name: "Getting Started",
        route: "getting_started",
        render: || html! { <GettingStarted /> },
    },
    DemoLink {
        name: "Init Theme",
        route: "init_theme",
        render: || html! { <InitThemeDemoSection /> },
    },
    DemoLink {
        name: "Icon",
        route: "icon",
        render: || html! { <IconDemoSection /> },
    },
];

pub const FORM_LINKS: &[DemoLink] = &[
    // Forms
    DemoLink {
        name: "Form",
        route: "form",
        render: || html! { <FormDemoSection /> },
    },
    DemoLink {
        name: "Input",
        route: "input",
        render: || html! { <InputDemoSection /> },
    },
    DemoLink {
        name: "Label",
        route: "label",
        render: || html! { <LabelDemoSection /> },
    },
    DemoLink {
        name: "Search Input",
        route: "search_input",
        render: || html! { <SearchInputDemoSection /> },
    },
    DemoLink {
        name: "Select",
        route: "select",
        render: || html! { <SelectDemoSection /> },
    },
    DemoLink {
        name: "Textarea",
        route: "textarea",
        render: || html! { <TextareaDemoSection /> },
    },
    DemoLink {
        name: "Form Modal",
        route: "form_modal",
        render: || html! { <FormModalDemoSection /> },
    },
    DemoLink {
        name: "Form Builder",
        route: "form_builder",
        render: || html! { <FormBuilderDemoSection /> },
    },
    DemoLink {
        name: "Checkbox",
        route: "checkbox",
        render: || html! { <CheckboxDemoSection /> },
    },
    DemoLink {
        name: "Switch",
        route: "switch",
        render: || html! { <SwitchDemoSection /> },
    },
    DemoLink {
        name: "Radio Group",
        route: "radio_group",
        render: || html! { <RadioGroupDemoSection /> },
    },
    DemoLink {
        name: "Color Input",
        route: "color_input",
        render: || html! { <ColorInputDemoSection /> },
    },
    DemoLink {
        name: "Phone Input",
        route: "phone_input",
        render: || html! { <PhoneInputDemoSection /> },
    },
    DemoLink {
        name: "Range Input",
        route: "range_input",
        render: || html! { <RangeInputDemoSection /> },
    },
    DemoLink {
        name: "File Input",
        route: "file_input",
        render: || html! { <FileInputDemoSection /> },
    },
    DemoLink {
        name: "JSON Input",
        route: "json_input",
        render: || html! { <JsonInputDemoSection /> },
    },
    DemoLink {
        name: "JSON-Backed Form",
        route: "json_backed_form",
        render: || html! { <JsonBackedFormDemoSection /> },
    },
    DemoLink {
        name: "Field Array",
        route: "field_array",
        render: || html! { <FieldArrayDemoSection /> },
    },
    DemoLink {
        name: "Key Value Input",
        route: "key_value_input",
        render: || html! { <KeyValueInputDemoSection /> },
    },
    DemoLink {
        name: "Tags Input",
        route: "tags_input",
        render: || html! { <TagsInputDemoSection /> },
    },
    DemoLink {
        name: "Static Values Input",
        route: "static_values_input",
        render: || html! { <StaticValuesInputDemoSection /> },
    },
];

pub const ATOM_LINKS: &[DemoLink] = &[
    // Atoms (basic UI elements)
    DemoLink {
        name: "A",
        route: "a",
        render: || html! { <AComponentDemoSection /> },
    },
    DemoLink {
        name: "Avatar",
        route: "avatar",
        render: || html! { <AvatarDemoSection /> },
    },
    DemoLink {
        name: "Badge",
        route: "badge",
        render: || html! { <BadgeDemoSection /> },
    },
    DemoLink {
        name: "Button",
        route: "button",
        render: || html! { <ButtonDemoSection /> },
    },
    DemoLink {
        name: "Chip",
        route: "chip",
        render: || html! { <ChipDemoSection /> },
    },
    DemoLink {
        name: "Typo",
        route: "typo",
        render: || html! { <TypoDemoSection /> },
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
        name: "Spacer",
        route: "spacer",
        render: || html! { <SpacerDemoSection /> },
    },
    DemoLink {
        name: "Image",
        route: "image",
        render: || html! { <ImageDemoSection /> },
    },
    DemoLink {
        name: "Video",
        route: "video",
        render: || html! { <VideoDemoSection /> },
    },
    DemoLink {
        name: "Linear Progress",
        route: "linear_progress",
        render: || html! { <LinearProgressDemoSection /> },
    },
    DemoLink {
        name: "Circular Progress",
        route: "circular_progress",
        render: || html! { <CircularProgressDemoSection /> },
    },
];

pub const MOLECULE_LINKS: &[DemoLink] = &[
    DemoLink {
        name: "Avatar Group",
        route: "avatar_group",
        render: || html! { <AvatarGroupDemoSection /> },
    },
    DemoLink {
        name: "Accordion",
        route: "accordion",
        render: || html! { <AccordionDemoSection /> },
    },
    DemoLink {
        name: "Breadcrumbs",
        route: "breadcrumbs",
        render: || html! { <BreadcrumbsDemoSection /> },
    },
    DemoLink {
        name: "Modal Button",
        route: "modal_button",
        render: || html! { <ModalButtonDemoSection /> },
    },
    DemoLink {
        name: "Copy to Clipboard",
        route: "copy_to_clipboard",
        render: || html! { <CopyToClipboardDemoSection /> },
    },
    DemoLink {
        name: "Download Button",
        route: "download_button",
        render: || html! { <DownloadButtonDemoSection /> },
    },
    DemoLink {
        name: "Tooltip",
        route: "tooltip",
        render: || html! { <TooltipDemoSection /> },
    },
    DemoLink {
        name: "Popover",
        route: "popover",
        render: || html! { <PopoverDemoSection /> },
    },
    DemoLink {
        name: "Hero Header",
        route: "hero_header",
        render: || html! { <HeroHeaderDemoSection /> },
    },
    DemoLink {
        name: "Modal",
        route: "modal",
        render: || html! { <ModalDemoSection /> },
    },
    DemoLink {
        name: "CodeBlock",
        route: "code_block",
        render: || html! { <CodeBlockDemoSection /> },
    },
];

pub const ORGANISM_LINKS: &[DemoLink] = &[
    DemoLink {
        name: "Card",
        route: "card",
        render: || html! { <CardDemoSection /> },
    },
    DemoLink {
        name: "Table",
        route: "table",
        render: || html! { <TableDemoSection /> },
    },
    DemoLink {
        name: "Nested List",
        route: "nested_list",
        render: || html! { <NestedListDemoSection /> },
    },
    DemoLink {
        name: "NavBar",
        route: "navbar",
        render: || html! { <NavBarDemoSection /> },
    },
    DemoLink {
        name: "App Bar",
        route: "app_bar",
        render: || html! { <AppBarDemoSection /> },
    },
    DemoLink {
        name: "Notification",
        route: "notification",
        render: || html! { <NotificationDemoSection /> },
    },
    DemoLink {
        name: "Stepper",
        route: "stepper",
        render: || html! { <StepperDemoSection /> },
    },
    DemoLink {
        name: "Tabs",
        route: "tabs",
        render: || html! { <TabsDemoSection /> },
    },
    DemoLink {
        name: "Markdown",
        route: "markdown",
        render: || html! { <MarkdownDemoSection /> },
    },
    DemoLink {
        name: "Sidebar",
        route: "sidebar",
        render: || html! { <SidebarDemoSection /> },
    },
];

pub const CHART_LINKS: &[DemoLink] = &[
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
        name: "Line Chart",
        route: "line_chart",
        render: || html! { <LineChartDemoSection /> },
    },
    DemoLink {
        name: "Pie Chart",
        route: "pie_chart",
        render: || html! { <PieChartDemoSection /> },
    },
    DemoLink {
        name: "Scatter Chart",
        route: "scatter_chart",
        render: || html! { <ScatterPlotDemoSection /> },
    },
];

pub fn all_demo_links() -> Vec<&'static DemoLink> {
    SYSTEM_LINKS
        .iter()
        .chain(ATOM_LINKS.iter())
        .chain(MOLECULE_LINKS.iter())
        .chain(ORGANISM_LINKS.iter())
        .chain(CHART_LINKS.iter())
        .chain(FORM_LINKS.iter())
        .collect()
}

fn build_nested_sidebar_links(_title: &str, links: &[DemoLink]) -> Vec<NestedItem> {
    links
        .iter()
        .map(|link| NestedItem::with_select(link.name, link.route))
        .collect()
}

#[component(DemoSidebar)]
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
                    open_text: html! { "System" },
                    icon: html! { <SystemIcon /> },
                    list: build_nested_sidebar_links("system", SYSTEM_LINKS),
                },
                SidebarButton {
                    open_text: html! { "Atoms" },
                    icon: html! { <AtomIcon /> },
                    list: build_nested_sidebar_links("atoms", ATOM_LINKS),
                },
                SidebarButton {
                    open_text: html! { "Molecules" },
                    icon: html! { <PolylineIcon /> },
                    list: build_nested_sidebar_links("molecules", MOLECULE_LINKS),
                },
                SidebarButton {
                    open_text: html! { "Organisms" },
                    icon: html! { <AppsIcon /> },
                    list: build_nested_sidebar_links("organisms", ORGANISM_LINKS),
                },
                SidebarButton {
                    open_text: html! { "Charts" },
                    icon: html! { <BarChartIcon /> },
                    list: build_nested_sidebar_links("charts", CHART_LINKS),
                },
                SidebarButton {
                    open_text: html! { "Forms" },
                    icon: html! { <FormIcon /> },
                    list: build_nested_sidebar_links("forms", FORM_LINKS),
                },
            ]}
        />
    }
}
