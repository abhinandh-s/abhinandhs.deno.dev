use yew::prelude::*;

#[function_component(About)]
pub fn about_page() -> Html {
    html! {
      <>
        <crate::components::header::Header />
        <crate::components::footer::Footer />
      </>
    }
}
