use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;

#[function_component(About)]
pub fn about_page() -> Html {
    html! {
        <>
            <crate::components::header::Header />
            <div
            id="wrapper"
            class="p-2 mx-auto max-w-3xl flex flex-col justify-center">

                <h1 class="border-l-4 border-l-just-red pl-4 font-bold max-tablet:text-3xl text-4xl mt-12">{ "Interests" }<span class="text-just-red">{"."}</span></h1>
                <div class="pt-8"></div>
                <p class="">
                    { "Some of my favorite movies and series are listed here: " }
                    <Link<Route>
                        to={Route::Movies}
                        classes="text-just-red dark:text-just-red underline transition-colors duration-200"
                    >
                        { "Movies" }
                    </Link<Route>>
                </p>

                <crate::components::footer::Footer />
            </div>
            </>
    }
}
