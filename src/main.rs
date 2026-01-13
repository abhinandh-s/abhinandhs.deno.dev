use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

use self::pages::post::Article;

mod articles;
mod pages;
mod route;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    )
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::home::HomePage /> },
        Route::NotFound => {
            html! {
                <>
                      <h1 class="text-4xl font-bold"> { "404 - Page not found" }</h1>
          <p class="my-4">
          { "The page you were looking for doesn't exist." }
          </p>
          <a href="/" class="underline">{ "Go back home" }</a>

                </>
            }
        }
        Route::Articles { id } => html! { <Article post_id={id} /> },
        Route::ArticlesRoute => {},
    }
}
