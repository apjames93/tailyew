html! {
    <div class="space-y-8">
        // 1. Navigation list with left icons
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Navigation list with left icons" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Use " } <code>{ "<Li>" }</code> { " for navigation-style rows with optional icons and state styles." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul
                    dense=true
                    spacing="space-y-1"
                >
                    <Li
                        active=true
                        icon={html! { <span class="material-icons text-primary">{ "inbox" }</span> }}
                    >
                        { "Active with left icon" }
                    </Li>
                    <Li
                        hover=true
                        icon={html! { <span class="material-icons text-gray-500">{ "mail" }</span> }}
                    >
                        { "Hoverable with left icon" }
                    </Li>
                    <Li
                        icon={html! { <span class="material-icons text-success">{ "check_circle" }</span> }}
                    >
                        { "Neutral with colored left icon" }
                    </Li>
                </Ul>
            </div>
        </Section>

        // 2. Right-aligned icons and bordered rows
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Right icons & bordered items" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Align icons to the right and add borders to create structured navigation or menus." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul
                    marker_type={MarkerType::Disc}
                    dense=true
                    spacing="space-y-1"
                >
                    <Li
                        bordered=true
                        icon_position={IconPosition::Right}
                        icon={html! { <span class="material-icons">{ "chevron_right" }</span> }}
                    >
                        { "Bordered with right icon" }
                    </Li>
                    <Li
                        icon_position={IconPosition::Right}
                        icon={html! { <span class="material-icons">{ "arrow_forward" }</span> }}
                    >
                        { "Neutral row with right icon" }
                    </Li>
                </Ul>
            </div>
        </Section>

        // 3. Clickable list item
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Clickable list items" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Attach click handlers for interactive lists, menus, and in-place navigation." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul
                    marker_type={MarkerType::Decimal}
                    dense=true
                    spacing="space-y-1"
                >
                    <Li
                        on_click={Callback::from(|_| {
                            web_sys::console::log_1(&"Item clicked!".into());
                        })}
                        icon_position={IconPosition::Right}
                        icon={html! { <span class="material-icons">{ "arrow_forward" }</span> }}
                    >
                        { "Clickable with right icon" }
                    </Li>
                </Ul>
            </div>
        </Section>
    </div>
}
