use yew::Classes;
use yew::classes;

pub(crate) fn field_array_root_class() -> Classes {
    classes!("space-y-3")
}

pub(crate) fn field_array_header_class() -> Classes {
    classes!("space-y-1")
}

pub(crate) fn field_array_title_class() -> Classes {
    classes!(
        "text-sm",
        "font-medium",
        "text-gray-900",
        "dark:text-gray-100"
    )
}

pub(crate) fn field_array_helper_class() -> Classes {
    classes!("text-xs", "text-gray-500", "dark:text-gray-400")
}

pub(crate) fn field_array_count_class() -> Classes {
    classes!(
        "text-xs",
        "font-normal",
        "text-gray-500",
        "dark:text-gray-400"
    )
}

pub(crate) fn field_array_body_class() -> Classes {
    classes!("space-y-3")
}

pub(crate) fn field_array_validation_summary_class() -> Classes {
    classes!(
        "rounded-md",
        "bg-red-50",
        "px-2",
        "py-1.5",
        "text-xs",
        "text-red-700",
        "dark:bg-red-950/50",
        "dark:text-red-200"
    )
}

pub(crate) fn field_array_list_class() -> Classes {
    classes!("space-y-3")
}

pub(crate) fn field_array_row_class(row_deleted: bool) -> Classes {
    classes!(
        "rounded-lg",
        "border",
        "p-3",
        "transition-colors",
        "sm:p-4",
        if row_deleted {
            "border-amber-200 bg-amber-50/70 opacity-80 dark:border-amber-800 dark:bg-amber-950/30"
        } else {
            "border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-900"
        }
    )
}

pub(crate) fn field_array_empty_state_class() -> Classes {
    classes!("py-3", "text-sm")
}

pub(crate) fn field_array_add_container_class() -> Classes {
    classes!("flex", "flex-wrap", "items-center", "gap-2", "pt-2")
}

pub(crate) fn field_array_add_action_class() -> Classes {
    classes!(
        "inline-flex",
        "h-8",
        "items-center",
        "gap-1.5",
        "px-1.5",
        "text-xs",
        "text-gray-600",
        "shadow-none",
        "hover:bg-gray-50",
        "dark:text-gray-300",
        "dark:hover:bg-gray-800"
    )
}

pub(crate) fn field_array_status_class(is_valid: bool) -> Classes {
    if is_valid {
        classes!("text-xs", "text-gray-500", "dark:text-gray-400")
    } else {
        classes!(
            "rounded-md",
            "bg-amber-50",
            "px-2",
            "py-1.5",
            "text-xs",
            "text-amber-800",
            "dark:bg-amber-950/50",
            "dark:text-amber-200"
        )
    }
}

pub(crate) fn field_array_preview_class() -> Classes {
    classes!(
        "rounded-md",
        "border",
        "border-gray-100",
        "bg-gray-50/60",
        "dark:border-gray-800",
        "dark:bg-gray-800/40"
    )
}

pub(crate) fn field_array_scalar_wrapper_class() -> Classes {
    classes!(
        "field-array-scalar",
        "[&>section]:rounded-none",
        "[&>section]:border-0",
        "[&>section]:bg-transparent",
        "[&>section]:shadow-none",
        "[&>section>div:first-child]:border-b-0",
        "[&>section>div:first-child]:px-0",
        "[&>section>div:first-child]:py-0",
        "[&>section>div:nth-child(2)]:px-0",
        "[&>section>div:nth-child(2)]:py-0"
    )
}

pub(crate) fn object_fields_grid_class() -> Classes {
    classes!("grid", "grid-cols-1", "gap-3", "sm:grid-cols-2")
}

pub(crate) fn object_row_action_class() -> Classes {
    classes!("mb-3", "flex", "justify-end")
}
