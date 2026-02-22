// src/components/theme.rs

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Flavour {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl Flavour {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Latte     => "latte",
            Self::Frappe    => "frappe",
            Self::Macchiato => "macchiato",
            Self::Mocha     => "mocha",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Latte     => "Latte",
            Self::Frappe    => "Frappé",
            Self::Macchiato => "Macchiato",
            Self::Mocha     => "Mocha",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Latte     => "☕",
            Self::Frappe    => "🍵",
            Self::Macchiato => "🌙",
            Self::Mocha     => "🌑",
        }
    }

    pub fn is_dark(self) -> bool {
        !matches!(self, Self::Latte)
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "latte"     => Some(Self::Latte),
            "frappe"    => Some(Self::Frappe),
            "macchiato" => Some(Self::Macchiato),
            "mocha"     => Some(Self::Mocha),
            _           => None,
        }
    }
}

const ALL_FLAVOURS: &[Flavour] = &[
    Flavour::Latte,
    Flavour::Frappe,
    Flavour::Macchiato,
    Flavour::Mocha,
];

const STORAGE_KEY: &str = "catppuccin-theme";

// ──────────────────────────────────────────────────────────────────────────────
// ── Helpers ───────────────────────────────────────────────────────────────────
// ──────────────────────────────────────────────────────────────────────────────

fn load_saved() -> Option<Flavour> {
    let storage = window()?.local_storage().ok()??;
    Flavour::from_str(&storage.get_item(STORAGE_KEY).ok()??)
}

fn save(flavour: Flavour) {
    if let Some(Ok(Some(s))) = window().map(|w| w.local_storage()) {
        let _ = s.set_item(STORAGE_KEY, flavour.as_str());
    }
}

fn os_prefers_dark() -> bool {
    window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

fn default_flavour() -> Flavour {
    if let Some(saved) = load_saved() { return saved; }
    if os_prefers_dark() { Flavour::Mocha } else { Flavour::Latte }
}

/// Apply theme to <html>.
/// Safe in both browser and Deno SSR — window() is None on the server,
/// so the whole function is a no-op there (SSR sends unstyled HTML and
/// the client hydrates with the correct theme immediately after).
fn apply(flavour: Flavour) {
    // window() returns None in Deno / non-browser WASM — bail out silently.
    let Some(win) = window() else { return };
    let Some(doc) = win.document() else { return };
    // document_element() returns Element, not HtmlElement.
    // We need HtmlElement for .style(), so we cast with dyn_into().
    let Some(html) = doc.document_element() else { return };

    // 1. data-theme → activates --ctp-* vars in your CSS.
    let _ = html.set_attribute("data-theme", flavour.as_str());

    // 2. Set color-scheme so Tailwind's `dark:` variant fires for
    //    Frappé and Macchiato too, not just Mocha.
    //    We cast Element → HtmlElement to access .style().
    //    This is the line that was failing before: the cast must be
    //    dyn_into (consuming) not dyn_ref (borrowing) because
    //    document_element() gives us an owned Element.
    let scheme = if flavour.is_dark() { "dark" } else { "light" };
    if let Ok(html_el) = html.dyn_into::<web_sys::HtmlElement>() {
        let _ = html_el.style().set_property("color-scheme", scheme);
    }
}

// ── Component ─────────────────────────────────────────────────────────────────

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let current = use_state(default_flavour);
    let open     = use_state(|| false);

    // Apply on mount + whenever flavour changes.
    {
        let current = current.clone();
        use_effect_with(*current, move |&f| {
            apply(f);
            || ()
        });
    }

    // Register a document click-listener only while the dropdown is open,
    // so outside clicks close it. Cleaned up automatically when closed.
    {
        let open = open.clone();
        use_effect_with(*open, move |&is_open| {
            if !is_open {
                return Box::new(|| ()) as Box<dyn FnOnce()>;
            }

            let open_ref = open.clone();
            let closure = Closure::<dyn Fn()>::new(move || open_ref.set(false));

            let doc = window().unwrap().document().unwrap();
            doc.add_event_listener_with_callback(
                "click",
                closure.as_ref().unchecked_ref(),
            ).unwrap();

            // Grab the raw JS function so we can remove it on cleanup.
            let cb = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
            closure.forget(); // keep alive until cleanup runs

            Box::new(move || {
                if let Some(doc) = window().and_then(|w| w.document()) {
                    let _ = doc.remove_event_listener_with_callback("click", &cb);
                }
            }) as Box<dyn FnOnce()>
        });
    }

    let toggle = {
        let open = open.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation(); // prevent the close-listener from firing
            open.set(!*open);
        })
    };

    // Inner clicks must not bubble out and trigger the close listener.
    let stop = Callback::from(|e: MouseEvent| e.stop_propagation());

    html! {
        <div class="relative" onclick={stop}>

            // ── Trigger ──────────────────────────────────────────────────────
            <button
                class="theme-switcher-btn"
                onclick={toggle}
                aria-label="Switch Catppuccin theme"
                aria-expanded={open.to_string()}
                aria-haspopup="listbox"
            >
                <span class="text-base leading-none">{ current.icon() }</span>
                <span>{ current.label() }</span>
                <svg
                    class={if *open {
                        "w-3.5 h-3.5 transition-transform duration-200 rotate-180"
                    } else {
                        "w-3.5 h-3.5 transition-transform duration-200"
                    }}
                    viewBox="0 0 12 12" fill="none" stroke="currentColor"
                    stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                >
                    <polyline points="2 4 6 8 10 4"/>
                </svg>
            </button>

            // ── Dropdown ─────────────────────────────────────────────────────
            if *open {
                <div
                    class="theme-switcher-dropdown"
                    role="listbox"
                    aria-label="Catppuccin flavour"
                >
                    <div class="px-3 pt-2.5 pb-1
                                text-[0.62rem] font-bold tracking-[0.12em] uppercase
                                text-[var(--ctp-overlay1)] select-none">
                        { "Catppuccin" }
                    </div>
                    <div class="mx-2 mb-1 border-t border-[var(--ctp-surface0)]"/>

                    { for ALL_FLAVOURS.iter().map(|&flavour| {
                        let current_h = current.clone();
                        let open_h    = open.clone();
                        let is_active = *current == flavour;

                        let select = Callback::from(move |_: MouseEvent| {
                            save(flavour);
                            current_h.set(flavour);
                            open_h.set(false);
                        });

                        html! {
                            <button
                                key={flavour.as_str()}
                                role="option"
                                aria-selected={is_active.to_string()}
                                class="theme-switcher-item w-full text-left"
                                onclick={select}
                            >
                                <span class="w-5 shrink-0 text-center text-base leading-none">
                                    { flavour.icon() }
                                </span>
                                <span class="flex flex-col leading-tight">
                                    <span>{ flavour.label() }</span>
                                    <span class="text-[0.63rem] text-[var(--ctp-overlay0)] font-normal">
                                        { if flavour.is_dark() { "dark" } else { "light" } }
                                    </span>
                                </span>
                                <span class="theme-check">
                                    <svg class="w-3.5 h-3.5" viewBox="0 0 12 12"
                                         fill="none" stroke="currentColor"
                                         stroke-width="2.5" stroke-linecap="round"
                                         stroke-linejoin="round">
                                        <polyline points="1 6 4.5 9.5 11 2.5"/>
                                    </svg>
                                </span>
                            </button>
                        }
                    }) }

                    <div class="mx-3 mt-1 mb-2.5 pt-1.5
                                border-t border-[var(--ctp-surface0)]
                                text-[0.6rem] text-center text-[var(--ctp-overlay0)]">
                        { "Saved to your browser" }
                    </div>
                </div>
            }
        </div>
    }
}
