#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub year: String,
    pub month: String,
    pub post_id: String,
}

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;
use crate::utils::{get_article_by_id, get_date, markdown_to_html};

#[function_component(ArticleEntryWithDate)]
pub fn article_entry_with_date(props: &ArticleProps) -> Html {
    match get_article_by_id(&props.post_id) {
        Some(article) => {
            let date_str = article.matter.published_at.as_str(); // e.g., "2024-05-12"
            let date_display = get_date(date_str, false);

            // Extract segments for the URL
            let parts: Vec<&str> = date_str.split('-').collect();
            let year = parts.first().unwrap_or(&"0000").to_string();
            let month = parts.get(1).unwrap_or(&"00").to_string();

            html! {
              <li class="border-t border-latte-text dark:border-mocha-text py-2">
                <Link<Route>
                    to={Route::Articles {
                        year: year.clone(),
                        month: month.clone(),
                        id: article.id.clone()
                    }}
                    classes="py-2 flex group gap-4"
                >
                    <div class="w-24 shrink-0"> { date_display } </div>
                    <div>
                        <h2 class="font-bold group-hover:underline">{ article.matter.title }</h2>
                        <p> { article.matter.snippet } </p>
                    </div>
                </Link<Route>>
              </li>
            }
        }
        None => html!(),
    }
}


#[function_component(ArticleIndex)]
pub fn article_index() -> Html {
    html! {
      <>
        <crate::components::header::Header />
        <div class="p-4 mx-auto max-w-3xl flex flex-col justify-center">
          <h1 class="font-bold text-5xl mt-12">{ "Abhi's Blog" }
              <span class="text-just-red">{ "." }</span>
          </h1>
          <ul class="mt-8">
            {
              for crate::utils::get_all_articles_sorted().into_iter().map(|article| {
                let parts: Vec<&str> = article.matter.published_at.split('-').collect();
                      html! {
                          <ArticleEntryWithDate
                              year={parts[0].to_string()}
                              month={parts[1].to_string()}
                              post_id={article.id}
                          />
                      }
              })
            }
          </ul>
          <div class="border-b broder-latte-text dark:border-mocha-text"></div>
          <crate::components::footer::Footer />
        </div>
      </>
    }
}

// https://abhinandh-s.github.io/#/articles/:post
//                                            ^
//                                            this page
#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    match get_article_by_id(&props.post_id) {
        Some(post) => {
            let html_content = markdown_to_html(&post.content);
            let ctx = Html::from_html_unchecked(html_content.into());
            let org = post.matter.published_at;
            let date = get_date(org.clone().as_str(), true);

            html! {
              <>
                <crate::components::header::Header />

                <div class="p-4 mx-auto max-w-3xl flex flex-col justify-center">
                  <h1 class="font-bold mt-12">{ date }</h1>
                  <h1 class="font-bold text-5xl mt-2">{ post.matter.title }</h1>

                  <div class="markdown-body mt-12">
                    { ctx }
                  </div>

                  <crate::components::footer::Footer />
                </div>
              </>
            }
        }
        None => html! { <crate::pages::_404::NotFound /> },
    }
}
