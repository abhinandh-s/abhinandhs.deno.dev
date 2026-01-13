#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub post_id: String,
}

use yew::prelude::*;

use crate::articles::{date_time, get_article_by_id, markdown_to_html};

#[function_component(ArticleEntry)]
pub fn article_entry(props: &ArticleProps) -> Html {
    match get_article_by_id(&props.post_id) {
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

#[function_component(ArticleEntryWithDate)]
pub fn article_entry_with_date(props: &ArticleProps) -> Html {
    match get_article_by_id(&props.post_id) {
        Some(article) => {
            html! {
                <li class="border-t py-2">
                    <a href={format!("/#/articles/{}", article.id)} class="py-2 flex group gap-4">
                    <div> { article.matter.published_at } </div>
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

#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    let post = get_article_by_id(&props.post_id);
    match post {
        Some(post) => {
            let html_content = markdown_to_html(&post.content);
            let ctx = Html::from_html_unchecked(html_content.into());
            let org = post.matter.published_at;
            let date = date_time(org.clone().as_str());

            html! {
                <>
                    <crate::pages::home::Header />
                    <div class="p-4 mx-auto max-w-3xl flex flex-col justify-center">
                    <h1 class="font-bold mt-12">{ date }</h1>
                    <h1 class="font-bold text-5xl mt-2">{ post.matter.title }</h1>
                    <div>

                    <div class="markdown-body mt-12">
                    { ctx }
                    </div>
                    </div>
                    <crate::pages::home::Footer />
                    </div>
                </>
            }
        }
        None => {
            html!(
            <>
                             <h1 class="text-4xl font-bold"> { "404 - Page not found" }</h1>
                 <p class="my-4">
                 { "The page you were looking for doesn't exist." }
                 </p>
                 <a href="/" class="underline">{ "Go back home" }</a>

                       </>

                   )
        }
    }
}
