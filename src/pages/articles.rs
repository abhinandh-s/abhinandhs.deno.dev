#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub year: String,
    pub month: String,
    pub post_id: String,
}

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;
use crate::utils::{TocItem, get_article_by_id, get_date, markdown_to_html};

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

#[derive(serde::Deserialize, PartialEq)]
struct QueryParams {
    tag: Option<String>,
}

#[function_component(ArticleIndex)]
pub fn article_index() -> Html {
    let search_query = use_state(String::new);
    let location = yew_router::hooks::use_location().unwrap();
    let all_articles = use_memo((), |_| crate::utils::get_all_articles_sorted());

    // EFFECT: Sync URL query params to the search state
    {
        let search_query = search_query.clone();
        use_effect_with(location, move |loc| {
            if let Ok(params) = loc.query::<QueryParams>()
                && let Some(tag) = params.tag
            {
                search_query.set(format!("#{}", tag));
            }
            || ()
        });
    }

    // For Tags
    let on_tag_click = {
        let search_query = search_query.clone();
        Callback::from(move |tag: String| {
            search_query.set(format!("#{}", tag)); // Prepend # for tag search
        })
    };

    // Filter logic
    let filtered_articles = {
        let query = (*search_query).to_lowercase();
        let articles = (*all_articles).clone();

        if query.is_empty() {
            articles
        } else if query.starts_with('#') {
            // Tag search logic
            let target = query.trim_start_matches('#');
            articles
                .into_iter()
                .filter(|a| {
                    a.matter
                        .tags
                        .as_ref()
                        .map(|t_list| t_list.iter().any(|t| t.to_lowercase() == target))
                        .unwrap_or(false)
                })
                .collect()
        } else {
            // Normal text search logic
            articles
                .into_iter()
                .filter(|a| {
                    a.matter.title.to_lowercase().contains(&query)
                        || a.matter.snippet.to_lowercase().contains(&query)
                })
                .collect()
        }
    };

    let on_input = {
        let search_query = search_query.clone();
        let navigator = yew_router::hooks::use_navigator().unwrap();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value();

            // If the user starts typing, remove the ?tag= from the URL
            if !val.starts_with('#') {
                navigator.replace(&Route::ArticlesRoute);
            }

            search_query.set(val);
        })
    };

    html! {
      <>
        <crate::components::header::Header />
        <div class="p-4 mx-auto max-w-3xl flex flex-col justify-center">
          <h1 class="font-bold text-5xl mt-12">
            { "Abhi's Blog" }<span class="text-just-red">{ "." }</span>
          </h1>

          // --- Search Bar ---
          <div class="mt-8 relative">
            <input
                type="text"
                placeholder="Search articles..."
                class="w-full bg-surface0 text-text p-3 rounded-lg border border-surface1 focus:border-just-red outline-none transition-all"
                oninput={on_input}
            />
            <span class="absolute right-3 top-3 text-subtext0">
                { format!("{} found", filtered_articles.len()) }
            </span>
          </div>

          <TagCloud on_tag_click={on_tag_click} />

          <ul class="mt-8">
            {
              for filtered_articles.clone().into_iter().map(|article| {
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

          if filtered_articles.is_empty() {
              <p class="text-center text-subtext0 mt-10">{"No articles match your search."}</p>
          }

          <div class="border-b border-surface1 mt-12"></div>
          <crate::components::footer::Footer />
        </div>
      </>
    }
}

// https://abhinandhs.deno.dev/articles/:post
//                                            ^
//                                            this page
#[function_component(Article)]
pub fn article(props: &ArticleProps) -> Html {
    let post_id = props.post_id.clone();

    let post = get_article_by_id(&post_id);

    // Dynamic SEO Update
    use_effect_with(post.clone(), |post| {
        if let Some(article) = post
            && let Some(window) = web_sys::window()
        {
            let document = window.document().unwrap();
            // Update Title
            document.set_title(&format!("{} | Abhi's Blog", article.matter.title));

            // Update Description meta tag
            if let Ok(Some(meta)) = document.query_selector("meta[name='description']") {
                let _ = meta.set_attribute("content", &article.matter.snippet);
            }
        }
        || ()
    });

    // This effect runs whenever the post_id changes
    // This solves the artice page scroll along with index problem
    use_effect_with(post_id.clone(), |_| {
        if let Some(window) = web_sys::window() {
            window.scroll_to_with_x_and_y(0.0, 0.0);
        }
        || () // Cleanup (not needed here)
    });

    let navigator = yew_router::hooks::use_navigator().unwrap();

    // Define what happens when a tag is clicked in the sidebar
    let on_tag_click = {
        let navigator = navigator.clone();
        Callback::from(move |tag: String| {
            // Redirect to home with a query parameter
            let _ = navigator.push_with_query(
                &Route::ArticlesRoute,
                &std::collections::HashMap::from([("tag", tag)]),
            );
        })
    };

    match get_article_by_id(&props.post_id) {
        Some(post) => {
            let word_count = &post.content.split_whitespace().count();
            let reading_time = (*word_count as f32 / 200.0).ceil();

            let (toc_items, html) = markdown_to_html(&post.content);
            let ctx = Html::from_html_unchecked(html.into());
            let org = post.matter.published_at;
            let date = get_date(org.clone().as_str(), true);

            let tags = post.matter.tags;
            // let c_tag_on_click = {
            //     let cb = props.on_tag_click.clone();
            //     let name = tag_name.clone();
            //     Callback::from(move |_| cb.emit(name.clone()))
            //     };

            html! {
                            <>
                              <crate::components::header::Header />

                              <div class="flex flex-col lg:flex-row relative max-w-7xl mx-auto w-full">
              <aside class="max-tablet:hidden w-64 flex-shrink-0 sticky top-20 self-start h-fit p-4">
                      <TableOfContents toc_items={toc_items} />
            <TagCloud on_tag_click={&on_tag_click} />
                  </aside>

                              <main class="flex-grow w-full max-w-3xl px-4 lg:px-8">
                      <p class="font-bold mt-12 text-mocha-overlay2">{ date }</p>
                      <h1 class="font-bold text-5xl mt-2 leading-tight">{ post.matter.title }</h1>

                      <p>{ format!("Reading Time: ~ {reading_time} minutes") }</p>
            <CTagCloud on_tag_click={on_tag_click} tags={tags} />

                      <div class="markdown mt-12 overflow-x-auto">
                          // ^ added overflow-x-auto to prevent wide code blocks from breaking mobile
                          { ctx }
                      </div>
                  </main>


                              </div>
                                <crate::components::footer::Footer />
                            </>
                          }
        }
        None => html! { <crate::pages::_404::NotFound /> },
    }
}

#[derive(Properties, PartialEq)]
pub struct TocProps {
    pub toc_items: Vec<TocItem>,
}

#[function_component(TableOfContents)]
pub fn table_of_contents(props: &TocProps) -> Html {
    html! {
            <nav class="toc-container p-4 bg-transparent">
                <h3 class="text-subtext1 font-bold mb-4 uppercase text-xs tracking-widest">{"On this page"}</h3>
                <ul class="space-y-2 list-none border-l border-surface1 ml-2">
                    { for props.toc_items.iter().map(|item| {
                        // Calculate indentation based on level (H1 = 0, H2 = 4, H3 = 8...)
                        // Note: In Tailwind, dynamic strings like ml-{x} must be in your safelist
                        // or handled via style if the value is truly dynamic.
                        let left_padding = format!("padding-left: {}rem", (item.level as f32 - 1.0) * 0.75);

                        html! {
                            <li key={item.id.clone()} style={left_padding}>
                                <a href={format!("#{}", item.id)}
       class="block py-1 text-subtext0 hover:text-just-red transition-all duration-200 text-sm border-l-2 border-transparent hover:border-just-red pl-2 -ml-[1px]">
        { &item.text }
    </a>
                            </li>
                        }
                    })}
                </ul>
            </nav>
        }
}

#[derive(Properties, PartialEq)]
pub struct TagCloudProps {
    #[prop_or_default]
    pub on_tag_click: Callback<String>,
}

#[function_component(TagCloud)]
pub fn tag_cloud(props: &TagCloudProps) -> Html {
    let tags_map = crate::utils::get_articles_by_tag();

    // Convert to Vec so we can sort
    let mut tags: Vec<_> = tags_map.into_iter().collect();
    
    // Sort ascending (A-Z)
    tags.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    html! {
        <div class="pb-4 mt-16">
            <h3 class="text-subtext1 font-bold mb-4 uppercase text-xs tracking-widest">{"Tags"}</h3>
            <div class="flex flex-wrap gap-2">
                { for tags.iter().map(|(tag, posts)| {
                    let tag_name = tag.clone();
                    let on_click = {
                        let cb = props.on_tag_click.clone();
                        let name = tag_name.clone();
                        Callback::from(move |_| cb.emit(name.clone()))
                    };

                    html! {
                        <span onclick={on_click}
                              class="px-3 py-1 bg-surface0 text-blue rounded-full text-xs border border-surface1 hover:border-blue cursor-pointer transition-all active:scale-95">
                            { format!("{} ({})", tag_name, posts.len()) }
                        </span>
                    }
                })}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CTagCloudProps {
    #[prop_or_default]
    pub on_tag_click: Callback<String>,
    tags: Option<Vec<String>>,
}
#[function_component(CTagCloud)]
pub fn c_article_tag_cloud(props: &CTagCloudProps) -> Html {
    let tags_map = props.tags.clone().unwrap_or_default();

    // Convert to Vec so we can sort
    let mut tags: Vec<_> = tags_map.into_iter().collect();
    
    // Sort ascending (A-Z)
    tags.sort_by_key(|a| a.to_lowercase());


    html! {
        <div class="pt-2">
            <div class="flex flex-wrap gap-2">
                { for tags.iter().map(|tag| {
                    let tag_name = tag.clone();
                    let on_click = {
                        let cb = props.on_tag_click.clone();
                        let name = tag_name.clone();
                        Callback::from(move |_| cb.emit(name.clone()))
                    };

                    html! {
                        <span onclick={on_click}
                              class="px-3 py-1 bg-surface0 text-blue rounded-full text-xs border border-surface1 hover:border-blue cursor-pointer transition-all active:scale-95">
                            { format!("{}", tag_name) }
                        </span>
                    }
                })}
            </div>
        </div>
    }
}
