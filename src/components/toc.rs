use yew::prelude::*;

use crate::utils::TocItem;

#[derive(Properties, PartialEq, Clone)]
pub struct TocProps {
    pub toc_items: Vec<TocItem>,
}

#[rustfmt::skip]
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
