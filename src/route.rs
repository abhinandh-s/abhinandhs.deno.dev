use yew_router::prelude::*;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/portfolio")]
    Portfolio,
    #[at("/articles")]
    ArticlesRoute,
    #[at("/articles/:id")]
    Articles { id: String },
    #[at("/about")]
    About,
    #[at("/movies")]
    Movies,
    #[not_found]
    #[at("/404")]
    NotFound,
}
