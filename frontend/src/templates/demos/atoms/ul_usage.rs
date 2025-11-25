html! {
    <div class="space-y-8">
        // Basic text list (no markers)
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Basic list (no markers)" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Use the default " } <code>{ "<Ul>" }</code> { " for simple stacked content without bullets." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul>
                    <Li>{ "No marker" }</Li>
                    <Li>{ "Another list item" }</Li>
                </Ul>
            </div>
        </Section>

        // Bulleted list (disc)
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Bulleted list" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Enable disc markers for standard bulleted lists, ideal for features or key points." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul marker_type={MarkerType::Disc}>
                    <Li>{ "Disc marker" }</Li>
                    <Li>{ "Another list item" }</Li>
                </Ul>
            </div>
        </Section>

        // Numbered list (decimal)
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Numbered list" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Use decimal markers when order matters, such as for steps or instructions." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul marker_type={MarkerType::Decimal}>
                    <Li>{ "Decimal style marker" }</Li>
                    <Li>{ "Item two" }</Li>
                </Ul>
            </div>
        </Section>

        // Custom marker color + spacing
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Custom marker color & spacing" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Customize marker color and vertical spacing to match your brand or layout." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul
                    marker_type={MarkerType::Decimal}
                    marker_color="marker:text-primary dark:marker:text-accent"
                    spacing="space-y-2"
                >
                    <Li>{ "Custom colored markers outside" }</Li>
                    <Li>{ "Works in light and dark" }</Li>
                </Ul>
            </div>
        </Section>

        // Dense variant
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Dense list" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Use the dense variant for navigation, menus, or compact sidebars." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul dense=true marker_type={MarkerType::Disc}>
                    <Li>{ "Dense list for compact layouts" }</Li>
                    <Li>{ "Great for nav or menus" }</Li>
                </Ul>
            </div>
        </Section>

        // Unstyled list for custom content
        <Section class="space-y-3">
            <Typo tag={TagType::H3} class="font-semibold">
                { "Unstyled list (custom content)" }
            </Typo>
            <Typo class="text-sm text-gray-500 dark:text-gray-400">
                { "Turn off markers and style each item manually for cards, menus, or complex layouts." }
            </Typo>

            <div class="rounded-lg border border-gray-200 dark:border-gray-800 bg-white/5 dark:bg-gray-900/40 p-4">
                <Ul marker_type={MarkerType::None}>
                    <Li class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">
                        { "No marker, styled manually" }
                    </Li>
                    <Li class="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded">
                        { "Second manual item" }
                    </Li>
                </Ul>
            </div>
        </Section>
    </div>
}