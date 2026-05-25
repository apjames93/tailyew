use tailyew::atoms::{Chip, ChipSize, ChipVariant};
use yew::prelude::*;

html! {
    <div class="flex flex-wrap items-center gap-3">
        <Chip>{ "Neutral" }</Chip>
        <Chip variant={ChipVariant::Primary}>{ "Primary" }</Chip>
        <Chip variant={ChipVariant::Success}>{ "Success" }</Chip>
        <Chip variant={ChipVariant::Warning}>{ "Warning" }</Chip>
        <Chip variant={ChipVariant::Danger}>{ "Danger" }</Chip>
        <Chip size={ChipSize::Small} removable={true} remove_aria_label="Remove beta">
            { "beta" }
        </Chip>
    </div>
}
