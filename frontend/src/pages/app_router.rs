use crate::templates::{DemoSidebar, NavBar};
use crate::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;

#[component(AppRouter)]
pub fn app_router() -> Html {
    html! {
        <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-800">

            // AppBar (fixed)
            <NavBar />

            // Sidebar Drawer + Buttons
            <DemoSidebar />

            // Main content area with sidebar offset + top padding
            <div class="flex-1 pt-20 pl-14"> // pl-14 = 56px to match sidebar toggle
                <div class="max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8">
                    <Switch<Route> render={switch} />
                </div>
            </div>

            // Optional Footer
            <div id="Footer"></div>
        </div>
    }
}
