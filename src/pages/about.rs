use yew::prelude::*;

#[function_component(About)]
pub fn about_page() -> Html {


    html! {
      <>
       <crate::components::header::Header />

            <div class="flex flex-wrap gap-6 justify-center p-8">
                <PinterestEmbed pin_id={BCS} />
                <PinterestEmbed pin_id="114208540547450434" />
                <PinterestEmbed pin_id="1548181157616156" />
            </div>

       <crate::components::footer::Footer />
      </>
    }
}

#[derive(Properties, PartialEq)]
pub struct PinterestProps {
    pub pin_id:AttrValue,
}

const BCS: &str = "4081455906989422";
const BB: &str = "114208540547450434";
const EL_C: &str = "1548181157616156";

#[function_component(PinterestEmbed)]
pub fn pinterest_embed(props: &PinterestProps) -> Html {
        // We set the container height to be shorter than the iframe to "crop" the name
    let container_style = "height: 500px; width: 345px; overflow: hidden; border-radius: 0px; margin: 5px";
    let iframe_style = "border: none; margin-top: 0px; border-radius: 0px; transform: scale(1.05);"; // zoomed a bit to make it square

    html! {
        <div style={container_style} class="m-10">
            <iframe 
                src={format!("https://assets.pinterest.com/ext/embed.html?id={}", props.pin_id)}
                height="617" // Full height including name 
                width="345" 
                style={iframe_style}
                scrolling="no"
            />
        </div>
    }
}
