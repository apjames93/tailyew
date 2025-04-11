// frontend/src/pages/app_router.rs

use crate::templates::NavBar;
use crate::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

/// Define a new component `AppRouter` that wraps the router and handles state management.
#[function_component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-800">

            // AppBar (fixed)
            <NavBar />

            // Main content
            <div
                class="flex-1"
                style="
                    padding-top: 74px;       /* nav height */
                    padding-left: 56px;      /* sidebar toggle bar (w-14 = 3.5rem = 56px) */
                "
            >
                <Switch<Route> render={switch} />
            </div>

            // Optional Footer
            <div id="Footer"></div>
        </div>
    }
}
