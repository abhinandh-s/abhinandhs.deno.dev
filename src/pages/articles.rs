#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub post_id: String,
}

use yew::prelude::*;

use crate::articles::{get_article_by_id, get_date, markdown_to_html};



#[function_component(ArticleEntryWithDate)]
pub fn article_entry_with_date(props: &ArticleProps) -> Html {
    match get_article_by_id(&props.post_id) {
        Some(article) => {
            let date = get_date(article.matter.published_at.as_str(), false);

            html! {
              <li class="border-t border-mocha-mantle dark:text-mocha-text py-2">
                <a href={format!("/#/articles/{}", article.id)} class="py-2 flex group gap-4">
                <div class="min-w-24"> { date } </div>
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

#[function_component(ArticleIndex)]
pub fn article_index() -> Html {
    html! {
      <>
        <crate::components::header::Header />
        <div class="p-4 mx-auto max-w-3xl flex flex-col justify-center">
        <h1 class="font-bold text-5xl mt-12">{ "Abhi's Blog" }<span class="text-just-red">
    { "." }
                    </span></h1>
          <ul class="mt-8">
            {
              for crate::articles::get_all_articles().into_iter().map(|articles| {
                html! { <ArticleEntryWithDate post_id={articles.id} /> }
              })
            }
          </ul>
          <crate::components::footer::Footer />
        </div>
      </>
    }
}

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
