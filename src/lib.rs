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

// #[function_component(App)]
// fn app() -> Html {
//     html!(
//         <BrowserRouter> // `HashRouter` is needed for github pages
//             <Switch<Route> render={switch} />
//         </BrowserRouter>
//     )
// }

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
 let history = AnyHistory::from(MemoryHistory::new());
    history.push(props.clone().path);


    html! {
    
        <Router history={history}>
            <Switch<Route> render={switch} />
            </Router>
    }
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

// #[allow(clippy::enum_variant_names)]
// #[derive(Clone, Routable, PartialEq)]
// pub enum Route {
//     #[at("/")]
//     Home,
//     #[at("/about")]
//     About,
// }
//
// fn switch(route: Route) -> Html {
//     match route {
//         Route::Home => html! {
//             <h1>{ "This is Home Page" }</h1>
//         },
//         Route::About => html! {
//             <h1>{ "This is About Page" }</h1>
//         },
//     }
// }
