use comrak::plugins::syntect::SyntectAdapterBuilder;
use serde::{Deserialize, Serialize};

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
    get_all_articles_sorted()
        .into_iter()
        .take(limit)
        .collect()
}

// input: `2026-01-12 21:34`
// return it as [`Monday, November 25, 2024`]
pub fn get_date(input: &str, long: bool) -> String {
    // 1. Parse the input string based on its format
    // %Y-%m-%d %H:%M matches "YYYY-MM-DD HH:MM"
    match chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d") {
    // 2. Format it to the desired output: "Monday, January 12, 2026"
    // %A = Full weekday, %B = Full month, %e = Day of month, %Y = Year
        Ok(date_time) => {
            match long {
                true => date_time.format("%A, %B %e, %Y").to_string(),
                false => date_time.format("%b %d, %Y").to_string(),
            }
            
        },
        Err(err) => err.to_string(),
    }
}

pub fn get_article_by_id(id: &str) -> Option<Article> {
    get_all_articles().into_iter().find(|f| f.id == id)
}

/*
 base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light
InspiredGitHub from here
Solarized (dark) and Solarized (light)
 */
pub fn markdown_to_html(source: &str) -> String {
    let adapter = SyntectAdapterBuilder::new().theme("base16-ocean.dark").build();
    let options = comrak::Options::default();
    let mut plugins = comrak::options::Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    comrak::markdown_to_html_with_plugins(source, &options, &plugins)
}
