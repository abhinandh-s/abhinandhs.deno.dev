use yew::prelude::*;

#[function_component(Movies)]
pub fn movies_page() -> Html {
    let pins = vec![
        "remember_me",
        "stay",
        "five_feet_apart",
        "before_sunrise",
        "before_sunset",
        "nightcrawler",
        "drive",
        "la_la_land",
        "blade_runner",
        "batman",
        "prisoners",
        "shutter_island",
        "ironman",
        "jhon_wick",
    ];

    html! {
        <>
            <crate::components::header::Header />
            <main class="w-full">
                // Responsive masonry columns
                <div style="columns: 300px;">
                    { for pins.into_iter().map(|id| html! {
                        <MovieEmbed path={id} />
                    }) }
                </div>
            </main>
            <crate::components::footer::Footer />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct MovieProps {
    pub path: AttrValue,
}

#[function_component(MovieEmbed)]
pub fn pinterest_embed(props: &MovieProps) -> Html {
    html! {
        <img src={format!("static/movies/{}.avif", &props.path)} alt="No Description" loading="lazy" class="max-w-full h-auto py-2" />
    }
}
