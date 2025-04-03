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
            // Navbar Component
            <NavBar />

            // Main content area with flex-grow to expand and fill available space
            <div class="flex-1" style="padding-top: 74px;">
                <Switch<Route> render={switch} />
            </div>

            // Footer Component, stays at the bottom
            <div id="Footer"></div>
        </div>
    }
}
