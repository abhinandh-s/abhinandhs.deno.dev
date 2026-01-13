use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    ArticlesRoute,
    #[at("/articles/:id")]
    Articles { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
