use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

pub const AGE: u8 = 22;
pub const ABOUT_ME: &str = "I'm currently a CMA student at Biswas Institute of Management Studies, Kerala, India. I spend some of my time messing around with computers and software. This site is a home for my psychological dysfunctioning. It's a place where I'm in control; no algorithms, no censorship, and no manipulation. Just raw thoughts and code.";

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
      <>
        <Header />
        <main>
        <div
         id="wrapper"
         class="p-2 mx-auto max-w-3xl flex flex-col justify-center">
           <h1 class="max-tablet:text-2xl text-4xl">{ "Hello, I'am" }</h1>
           <h1 class="max-tablet:text-5xl text-6xl font-extrabold">
             {"Abhinandh S"}
             <span class="text-just-red">{"."}</span>
           </h1>
           <h1 class="pt-8 text-2xl font-sans font-bold">{ "Welcome to my corner of Internet"}<span class="text-just-red">{"."}</span></h1>
           <h1 class="border-l-4 border-l-just-red pl-4 font-bold max-tablet:text-3xl text-4xl mt-12">{ "About Me" }<span class="text-just-red">{"."}</span></h1>
           <br />
          <p class="pt-3">{ format!("I am a {AGE}-years-old guy from India. {ABOUT_ME}") }</p>
           <h1 class="border-l-4 border-l-just-red pl-4 font-bold max-tablet:text-3xl text-4xl mt-12">{ "Recent Posts"}<span class="text-just-red">{ "." }</span></h1>
           <ul class="mt-8">
             {
               for crate::utils::get_recently_add(4).into_iter().map(|article| {
                 let parts: Vec<&str> = article.matter.published_at.split('-').collect();
                  html! {
                      <crate::pages::articles::ArticleEntryWithDate
                          year={parts[0].to_string()}
                      month={parts[1].to_string()}
                      post_id={article.id}
                      />
                  }
               })
             }
          </ul>
          <div class="border-b broder-latte-text dark:border-mocha-text"></div>
          <Footer />
          </div>
        </main>
      </>
    }
}
