use route::Route;
use wasm_bindgen::prelude::*;
use yew::{LocalServerRenderer, prelude::*};
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use self::pages::articles::Article;

mod components;
mod pages;
mod route;
mod utils;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <pages::home::HomePage /> },
        Route::Portfolio => html! { <pages::portfolio::Portfolio /> },
        Route::Articles { id } => html! { <Article post_id={id} /> },
        Route::ArticlesRoute => html! { <pages::articles::ArticleIndex /> },
        Route::About => html! { <pages::about::About /> },
        Route::NotFound => html! { <pages::_404::NotFound /> },
    }
}

#[function_component(App)]
fn app(props: &AppProps) -> Html {
    // 1. Logic outside the html! macro
    let content = if !props.path.is_empty() {
        // SERVER PATH: Use the provided path from Deno
        let history = AnyHistory::from(MemoryHistory::new());
        history.push(&props.path);
        html! {
            <Router history={history}>
                <Switch<Route> render={switch} />
            </Router>
        }
    } else {
        // BROWSER PATH: Use the URL in the address bar
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    };

    // 2. Return the result
    content
}

#[derive(Clone, PartialEq, Properties)]
pub struct AppProps {
    pub path: String,
}

#[wasm_bindgen]
pub async fn render(path: String) -> String {
       let renderer = LocalServerRenderer::<App>::with_props(AppProps { path });
    renderer.render().await
}

#[wasm_bindgen(start)]
pub fn run_app() {
    // Check if we are in a browser environment
    let is_browser = web_sys::window()
        .map(|w| w.document().is_some())
        .unwrap_or(false);

    if is_browser {
        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(root) = document.get_element_by_id("app") {
            yew::Renderer::<App>::with_root_and_props(
                root, 
                AppProps { path: String::new() }
            ).hydrate();
        }
    }
    // If not in browser (i.e., Deno), do nothing and let 
    // the 'render' function be called manually.
}
