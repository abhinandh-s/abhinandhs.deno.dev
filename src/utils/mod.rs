use std::collections::HashMap;

use comrak::plugins::syntect::SyntectAdapterBuilder;
use serde::{Deserialize, Serialize};
use syntect::highlighting::ThemeSet;

mod generated;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub matter: FrontMatter,
    pub content: String,
}

// The default `Pod` data type can be a bit unwieldy, so
// you can also deserialize it into a custom struct
#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FrontMatter {
    pub title: String,
    pub published_at: String,
    pub snippet: String,
    pub tags: Option<Vec<String>>,
}

pub fn get_all_articles() -> Vec<Article> {
    let mut articles = Vec::new();
    let mut dbg = String::new();

    for (id, ctx) in generated::ARTICLES.to_owned() {
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        match matter.parse::<FrontMatter>(ctx) {
            Ok(result) => {
                articles.push(Article {
                    id: id.to_owned(),
                    matter: result.data.unwrap_or_default(),
                    content: result.content,
                });
            }
            Err(err) => dbg.push_str(err.to_string().as_str()),
        }
    }
    articles
}

pub fn get_all_articles_sorted() -> Vec<Article> {
    let mut articles = get_all_articles();

    // Sort by date in descending order (latest first)
    articles.sort_by(|a, b| {
        // We compare b to a to achieve descending order
        b.matter.published_at.cmp(&a.matter.published_at)
    });

    articles
}

// For home page
pub fn get_recently_add(limit: usize) -> Vec<Article> {
    get_all_articles_sorted().into_iter().take(limit).collect()
}

// input: `2026-01-12 21:34`
// return it as [`Monday, November 25, 2024`]
pub fn get_date(input: &str, long: bool) -> String {
    // 1. Parse the input string based on its format
    // %Y-%m-%d %H:%M matches "YYYY-MM-DD HH:MM"
    match chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        // 2. Format it to the desired output: "Monday, January 12, 2026"
        // %A = Full weekday, %B = Full month, %e = Day of month, %Y = Year
        Ok(date_time) => match long {
            true => date_time.format("%A, %B %e, %Y").to_string(),
            false => date_time.format("%b %d, %Y").to_string(),
        },
        Err(err) => err.to_string(),
    }
}

pub fn get_article_by_id(id: &str) -> Option<Article> {
    get_all_articles().into_iter().find(|f| f.id == id)
}

pub fn get_articles_by_tag() -> HashMap<String, Vec<Article>> {
    let mut tag_map: HashMap<String, Vec<Article>> = HashMap::new();
    let articles = get_all_articles();

    for article in articles {
        if let Some(tags) = &article.matter.tags {
            for tag in tags {
                tag_map
                    .entry(tag.clone())
                    .or_default()
                    .push(article.clone());
            }
        }
    }
    tag_map
}

/*
base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light
InspiredGitHub from here
Solarized (dark) and Solarized (light)
*/
pub fn markdown_to_html(source: &str) -> (Toc, String) {
    let latte = (
        "latte",
        include_str!("../../static/themes/Catppuccin Latte.tmTheme"),
    );
    let frappe = (
        "frappe",
        include_str!("../../static/themes/Catppuccin Frappe.tmTheme"),
    );
    let macchiato = (
        "macchiato",
        include_str!("../../static/themes/Catppuccin Macchiato.tmTheme"),
    );
    let mocha = (
        "mocha",
        include_str!("../../static/themes/Catppuccin Mocha.tmTheme"),
    );

    // 2. Create a ThemeSet and add your theme to it
    let mut themeset = ThemeSet::load_defaults();

    for (name, theme) in [latte, frappe, macchiato, mocha] {
        let mut cursor = std::io::Cursor::new(theme);
        let custom_theme =
            ThemeSet::load_from_reader(&mut cursor).expect("Failed to parse theme file");

        themeset
            .themes
            .insert(format!("catppuccin-{name}"), custom_theme);
    }

    let adapter = SyntectAdapterBuilder::new()
        .theme_set(themeset) // Use the set containing your theme
        .theme("catppuccin-mocha") // Select it by the key used above
        // .theme_set(th_set)
        .build();

    let mut options = comrak::Options::default();
    options.extension.strikethrough = true;
    options.extension.header_ids = Some("md-heading-".to_string());
    options.extension.alerts = true;
    options.extension.tasklist = true;
    options.extension.spoiler = true;

    let mut plugins = comrak::options::Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let arena = comrak::Arena::new();
    let root = comrak::parse_document(&arena, source, &options);

    // Helper to extract text from a node and its children
    fn collect_text<'a>(node: &'a comrak::nodes::AstNode<'a>, output: &mut String) {
        match node.data.borrow().value {
            comrak::nodes::NodeValue::Text(ref t) => output.push_str(t),
            comrak::nodes::NodeValue::Code(ref c) => output.push_str(&c.literal),
            _ => {
                for child in node.children() {
                    collect_text(child, output);
                }
            }
        }
    }

    let mut toc = Toc::new();

    for node in root.children() {
        if let comrak::nodes::NodeValue::Heading(heading) = &node.data.borrow().value {
            let mut text = String::new();
            collect_text(node, &mut text);

            // Generate the ID. Comrak's default slugifier:
            // 1. Lowercase
            // 2. Remove non-alphanumeric (except hyphens/spaces)
            // 3. Replace spaces with hyphens
            let slug = text
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
                .replace(' ', "-");

            let id = format!("{}{}", "md-heading-", slug);

            toc.push(TocItem {
                level: heading.level,
                text,
                id,
            });
        }
    }

    let mut html_output = String::new();
    match comrak::format_html_with_plugins(root, &options, &mut html_output, &plugins) {
        Ok(_) => (),
        Err(err) => {
            html_output.push_str(err.to_string().as_str());
        }
    }

    (toc, html_output)
}

#[derive(PartialEq, Clone)]
pub struct TocItem {
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub type Toc = Vec<TocItem>;
