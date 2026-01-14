use yew::prelude::*;

use crate::articles::get_recently_add;
use crate::components::footer::Footer;
use crate::components::header::Header;

pub const AGE: u8 = 22;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let about = format!(
        "I'm Abhinandh S. I am a {AGE} years old guy from India, who loves computers and softwares.<br />I'm a CMA student at Biswas Institute of Management Studies, Kerala, India"
    );
    html! {
      <>
        <Header />
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
           <p>{ about }</p>
           <p>{ "This place is home for all my psychological dysfunctioning. A place where I am in control, with no censorship or manupilation." }</p>

                <div id="list articles" class="">
                  <h1 class="border-l-4 border-l-just-red pl-4 font-bold max-tablet:text-3xl text-4xl mt-12">
    { "Recent Posts"}
                    <span class="text-just-red">
    { "." }
                    </span>
                  </h1>
                  <ul class="mt-8">
                  {
                    for get_recently_add(5).into_iter().map(|articles| {
                        html! { <ArticleEntry post_id={articles.id} /> }
                    })
                  }
                  </ul>
                </div>

    <div className="border-b broder-latte-text dark:border-mocha-text">
                </div>

               <Footer />
                </div>
            </>
        }
}

#[function_component(ArticleEntry)]
pub fn article_entry(props: &super::articles::ArticleProps) -> Html {
    match crate::articles::get_article_by_id(&props.post_id) {
        Some(article) => {
            html! {
              <li class="border-t py-2">
                <a href={format!("/#/articles/{}", article.id)} class="py-2 flex group gap-4">
                <div>
                  <h2 class="font-bold group-hover:underline">{ article.matter.title }</h2>
                  <p class="text-gray-600"> { article.matter.snippet } </p>
                </div>
                </a>
              </li>
            }
        }
        None => html!(),
    }
}
