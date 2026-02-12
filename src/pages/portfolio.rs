use yew::prelude::*;

#[function_component(Portfolio)]
pub fn portfolio_page() -> Html {
    html! {
      <>
       <crate::components::header::Header />
         <Counter />
       <crate::components::footer::Footer />
      </>
    }
}

#[function_component(Counter)]
pub fn counter() -> Html {
    let count = use_state(|| 0);

    let onclick = {
        let count = count.clone();
        Callback::from(move |_| {
            count.set(*count + 1);
        })
    };

    html! {
        <div class="flex flex-col items-center p-4">
            <p class="text-xl mb-2">{"Count: "} {*count}</p>
            <button
                {onclick}
                class="px-4 py-2 bg-mocha-blue text-mocha-crust rounded hover:bg-frappe-blue transition-colors"
            >
                { "Increment" }
            </button>
        </div>
    }
}
