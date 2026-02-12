use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

const LINK_CLASS: &str = "hover:text-just-red aria-[current]:text-just-red";
const MOBILE_LINK_CLASS: &str = "block py-2 px-4 hover:text-just-red aria-[current]:text-just-red";

#[rustfmt::skip]
#[function_component(Header)]
pub fn header() -> Html {
    // State for handling the mobile menu toggle
    let is_menu_open = use_state(|| false);

    // Toggle menu visibility
    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    let mobile_menu_class = if *is_menu_open {
        "md:hidden flex flex-col items-center"
    } else {
        "hidden md:hidden flex flex-col items-center"
    };

    html! {
      <div class="font-bold">
        <nav class="w-full min-h-32 max-tablet:min-h-16 top-0 left-0 z-10">
          <div class="mx-auto px-4">
            <div class="flex justify-end items-center pt-8 max-tablet:py-4">

              /* Desktop Menu */
              <div class="flex max-tablet:hidden space-x-16 mt-12 pb-7 px-16">
                <Link<Route> to={Route::Home} classes={LINK_CLASS}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::Portfolio} classes={LINK_CLASS}>{ "Portfolio" }</Link<Route>>
                <Link<Route> to={Route::ArticlesRoute} classes={LINK_CLASS}>{ "Articles" }</Link<Route>>
                <Link<Route> to={Route::About} classes={LINK_CLASS}>{ "About" }</Link<Route>>
              </div>

              /* Mobile Hamburger Menu */
              <div class="hidden max-tablet:flex">
                <div class="pr-3"></div>
                <button onclick={toggle_menu} class="focus:outline-none" aria-label="Open Menu">
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    class="feather feather-menu"
                  >
                    <line x1="3" y1="12" x2="21" y2="12"></line>
                    <line x1="3" y1="6" x2="21" y2="6"></line>
                    <line x1="3" y1="18" x2="21" y2="18"></line>
                  </svg>
                </button>
              </div>
            </div>

            /* Mobile Menu Items */
            <div class={mobile_menu_class}>
              <Link<Route> to={Route::Home} classes={MOBILE_LINK_CLASS}>{ "Home" }</Link<Route>>
              <Link<Route> to={Route::Portfolio} classes={MOBILE_LINK_CLASS}>{ "Portfolio" }</Link<Route>>
              <Link<Route> to={Route::ArticlesRoute} classes={MOBILE_LINK_CLASS}>{ "Articles" }</Link<Route>>
              <Link<Route> to={Route::About} classes={MOBILE_LINK_CLASS}>{ "About" }</Link<Route>>
            </div>
          </div>
        </nav>
      </div>
    }
}
