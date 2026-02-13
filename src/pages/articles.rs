#[derive(Properties, PartialEq)]
pub struct ArticleProps {
    pub year: String,
    pub month: String,
    pub post_id: String,
}

use wasm_bindgen::prelude::Closure;
use web_sys::Element;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;
use crate::utils::{get_article_by_id, get_date, markdown_to_html};

#[rustfmt::skip]
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

#[derive(Properties, PartialEq)]
pub struct SearchBarProps {
    pub value: String,
    pub on_input: Callback<InputEvent>,
    pub on_clear: Callback<MouseEvent>,
    pub result_count: usize,
}

#[function_component(SearchBar)]
fn search_bar(props: &SearchBarProps) -> Html {
    html! {
    // --- Search Bar ---
              <div class="mt-8 relative">
                <input
                    type="text"
                    placeholder="Search articles..."
                    value={props.value.clone()}
                    class="w-full bg-surface0 text-text p-3 rounded-lg border border-surface1 focus:border-just-red outline-none transition-all"
                    oninput={props.on_input.clone()}
                />

                               if !props.value.is_empty() {
                                   <span class="absolute right-12 top-3">
                                   { format!("{} found", props.result_count) }
                                   </span>
                    <button
                        onclick={props.on_clear.clone()}
                        class="absolute right-3 top-3 text-subtext0 hover:text-just-red transition-colors"
                    >
                        // A simple "X" icon
                    <svg xmlns="http://www.w3.org/2000/svg"
                        width="24"
                        height="24"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        class="feather feather-x-square">
                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect>
                        <line x1="9" y1="9" x2="15" y2="15"></line>
                        <line x1="15" y1="9" x2="9" y2="15"></line>
                    </svg>
                    </button>
                }
              </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct EmptyStateProps {
    pub on_reset: Callback<MouseEvent>,
}

#[function_component(EmptyState)]
fn empty_state(props: &EmptyStateProps) -> Html {
    html! {
        <div class="text-center py-20">
            <p class="text-subtext0 mb-4 text-lg">{"No articles match your search."}</p>
            <button
                onclick={props.on_reset.clone()}
                class="px-6 py-2 bg-surface1 hover:bg-just-red hover:text-base00 rounded-full transition-all duration-300 font-medium"
            >
                {"Reset all filters"}
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ArticleListProps {
    pub articles: Vec<crate::utils::Article>, // Adjust type path as needed
}

#[function_component(ArticleList)]
fn article_list(props: &ArticleListProps) -> Html {
    html! {
        <ul class="mt-8">
            { for props.articles.iter().map(|article| {
                let parts: Vec<&str> = article.matter.published_at.split('-').collect();
                html! {
                    <ArticleEntryWithDate
                        key={article.id.clone()}
                        year={parts.first().unwrap_or(&"").to_string()}
                        month={parts.get(1).unwrap_or(&"").to_string()}
                        post_id={article.id.clone()}
                    />
                }
            })}
        </ul>
    }
}

#[rustfmt::skip]
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

    let navigator = yew_router::hooks::use_navigator().unwrap();
    let on_clear = {
        let search_query = search_query.clone();
        let navigator =  navigator.clone();
        Callback::from(move |_| {
            search_query.set(String::new());
            navigator.replace(&Route::ArticlesRoute);
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

          <SearchBar
            value={(*search_query).clone()} 
            on_input={on_input} 
            on_clear={on_clear.clone()} 
            result_count={filtered_articles.len()}
          />

          <TagCloud on_tag_click={on_tag_click} />

          if filtered_articles.is_empty() {
            <EmptyState on_reset={on_clear} />
          } else {
            <ArticleList articles={filtered_articles} />
          }

          <div class="border-b border-surface1"></div>
          <crate::components::footer::Footer />
        </div>
      </>
    }
}

// https://abhinandhs.deno.dev/articles/:post
//                                            ^
//                                            this page
#[rustfmt::skip]
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
    <CodeCopyManager />
                                <ReadingProgressBar />
                              
                              <crate::components::header::Header />

                              <div class="flex flex-col lg:flex-row relative max-w-7xl mx-auto w-full">
              <aside class="max-tablet:hidden w-64 flex-shrink-0 sticky top-20 self-start h-fit p-4">
                 //     <TableOfContents  />
       <crate::components::toc::TableOfContents toc_items={toc_items} />
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
pub struct TagCloudProps {
    #[prop_or_default]
    pub on_tag_click: Callback<String>,
}

#[rustfmt::skip]
#[function_component(TagCloud)]
pub fn tag_cloud(props: &TagCloudProps) -> Html {
    let tags_map = crate::utils::get_articles_by_tag();

    // Convert to Vec so we can sort
    let mut tags: Vec<_> = tags_map.into_iter().collect();
    
    // Sort ascending (A-Z)
    tags.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    html! {
        <div class="pb-4 mt-8">
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

#[rustfmt::skip]
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

use wasm_bindgen::JsCast;

#[function_component(ReadingProgressBar)]
fn reading_progress_bar() -> Html {
    let progress = use_state(|| 0.0);

    {
        let progress = progress.clone();
        use_effect_with((), move |_| {
            let raf_handle = std::rc::Rc::new(std::cell::RefCell::new(None::<i32>));

            let window = web_sys::window().expect("window not found");
            let document_element = window.document().unwrap().document_element().unwrap();

            let closure_handle = std::rc::Rc::new(std::cell::RefCell::new(None));

            let listener = {
                let progress = progress.clone();
                let raf_handle = raf_handle.clone();
                let document_element = document_element.clone();
                let closure_handle = closure_handle.clone();

                move |_: web_sys::Event| {
                    if raf_handle.borrow().is_some() {
                        return;
                    }

                    let progress = progress.clone();
                    let raf_handle_inner = raf_handle.clone();
                    let document_element = document_element.clone();

                    // Create the closure for rAF
                    let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(move |_| {
                        *raf_handle_inner.borrow_mut() = None;

                        let scroll_top = document_element.scroll_top() as f64;
                        let scroll_height = document_element.scroll_height() as f64;
                        let client_height = document_element.client_height() as f64;

                        let total = scroll_height - client_height;
                        if total > 0.0 {
                            progress.set((scroll_top / total) * 100.0);
                        }
                    })
                        as Box<dyn FnMut(f64)>);

                    let window = web_sys::window().unwrap();
                    let handle = window
                        .request_animation_frame(closure.as_ref().unchecked_ref())
                        .unwrap();

                    *raf_handle.borrow_mut() = Some(handle);
                    // Keep the closure alive until the next frame
                    *closure_handle.borrow_mut() = Some(closure);
                }
            };

            let scroll_closure = wasm_bindgen::prelude::Closure::wrap(
                Box::new(listener) as Box<dyn FnMut(web_sys::Event)>
            );

            window
                .add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref())
                .unwrap();

            move || {
                let window = web_sys::window().unwrap();
                window
                    .remove_event_listener_with_callback(
                        "scroll",
                        scroll_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                if let Some(handle) = *raf_handle.borrow() {
                    window.cancel_animation_frame(handle).unwrap();
                }
            }
        });
    }

    html! {
        <div class="fixed top-0 left-0 w-full h-1 z-[100] pointer-events-none">
            <div
                class="h-full bg-just-red transition-all duration-150 ease-out"
                style={format!("width: {}%; will-change: width; box-shadow: 0 0 8px rgba(237, 135, 150, 0.6);", *progress)}
            />
        </div>
    }
}

use web_sys::HtmlButtonElement;

#[function_component(CodeCopyManager)]
pub fn code_copy_manager() -> Html {
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // 1. Initial Injection Loop (Buttons + Labels)
        let pre_tags = document.query_selector_all(".markdown pre").unwrap();
        for i in 0..pre_tags.length() {
            if let Some(node) = pre_tags.get(i) {
                let pre = node.dyn_into::<Element>().unwrap();
                let _ = pre.class_list().add_1("group"); // Ensure group hover works

                if let Ok(Some(code)) = pre.query_selector("code") {
                    // Extract Language Label
                    let class_name = code.class_name();
                    let lang = class_name
                        .split_whitespace()
                        .find(|c| c.to_lowercase().starts_with("language-"))
                        .map(|f| f.replace("language-", ""))
                        .unwrap_or_default();

                    // Inject Label if missing
                    if !lang.is_empty() && pre.query_selector(".code-lang-label").unwrap().is_none() {
                        let lang_label = document.create_element("span").unwrap();
                        lang_label.set_class_name("code-lang-label");
                        lang_label.set_inner_html(&lang);
                        let _ = pre.append_child(&lang_label);
                    }
                }

                // Inject Copy Button if missing
                if pre.query_selector(".copy-button").unwrap().is_none() {
                    let btn = document.create_element("button").unwrap();
                    btn.set_class_name("copy-button");
                    btn.set_inner_html("Copy");
                    let _ = pre.append_child(&btn);
                }
            }
        }

        // 2. Setup the click listener
        // We clone `document` here so it can be moved into the closure
        let listener = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            let target = e.target().unwrap().dyn_into::<Element>().unwrap();

            if target.class_name().contains("copy-button") {
                let btn = target.dyn_into::<HtmlButtonElement>().unwrap();
                let pre = btn.parent_element().unwrap();

                if let Ok(Some(code)) = pre.query_selector("code") {
                    let text = code.text_content().unwrap_or_default();
                    let nav = web_sys::window().unwrap().navigator();
                    let _ = nav.clipboard().write_text(&text);

                    btn.set_inner_html("Copied!");
                    btn.class_list().add_1("copied").unwrap();

                    let btn_clone = btn.clone();
                    let reset_closure = Closure::wrap(Box::new(move || {
                        btn_clone.set_inner_html("Copy");
                        btn_clone.class_list().remove_1("copied").unwrap();
                    }) as Box<dyn FnMut()>);

                    let _ = web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                        reset_closure.as_ref().unchecked_ref(),
                        2000,
                    );
                    reset_closure.forget();
                }
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);

        // 3. Attach using the original `document` (which was not moved)
        document
            .add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
            .unwrap();

        let doc_for_cleanup = document.clone();
        move || {
            doc_for_cleanup
                .remove_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    });

    html! {}
}
