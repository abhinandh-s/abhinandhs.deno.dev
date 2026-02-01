use std::{
    fs::{self, File, read_to_string},
    io::Write,
    path::Path,
};

fn main() {
    let articles_dir = Path::new("articles/published");
    let out_file = Path::new("src/utils/generated.rs");

    println!("cargo:rerun-if-changed=articles/published");

    let mut entries = fs::read_dir(articles_dir)
        .expect("Failed to read articles directory")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.path());

    let mut file = File::create(out_file).expect("Failed to create generated.rs");

    writeln!(file, "// AUTO-GENERATED — DO NOT EDIT\n").unwrap();
    /*
    pub const ARTICLES: &[(&str, &str)] = &[
        ("post-01", include_str!("post-01.md")),
       // ("post-02", include_str!("post-02.md")),
       // ("post-03", include_str!("post-03.md")),
    ];
        */
    writeln!(file, "pub const ARTICLES: &[(&str, &str)] = &[").unwrap();
    for i in entries {
        writeln!(
            file,
            "   (\"{}\", include_str!(\"../../articles/published/{}\")),",
            i.path()
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .as_str(),
            i.path()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
                .as_str()
        )
        .unwrap();
    }
    writeln!(file, "];").unwrap();

    let _e = get_all_articles();

    let out_file = Path::new("static/articles/feed.json");
    let mut json_feed = File::create(out_file).expect("Failed to create feed.json");
    write!(json_feed, "{}", generate_json_feed()).unwrap();

    fs::write("static/articles/feed.xml", generate_rss_feed()).unwrap();

    fs::write("static/articles/feed.atom.xml", generate_atom_feed()).unwrap();
}

use serde::{Deserialize, Serialize};

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

pub enum FileName {
    Stem,
    Ext,
    Full,
}

pub fn path_as_string(path: &Path, f: FileName) -> String {
    match f {
        FileName::Stem => path
            .file_stem()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default(),
        FileName::Ext => path
            .extension()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default(),
        FileName::Full => path
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default(),
    }
}

pub fn get_all_articles() -> Vec<Article> {
    let mut articles = Vec::new();
    let mut dbg = String::new();
    let articles_dir = Path::new("articles/published");
    let mut entries = fs::read_dir(articles_dir)
        .expect("Failed to read articles directory")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .collect::<Vec<_>>();

    entries.sort_by_key(|e| e.path());

    for i in entries {
        let ctx =
            read_to_string(i.path().to_string_lossy().to_string().as_str()).unwrap_or_default();
        let matter = gray_matter::Matter::<gray_matter::engine::YAML>::new();
        match matter.parse::<FrontMatter>(&ctx) {
            Ok(result) => {
                articles.push(Article {
                    id: i
                        .path()
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    matter: result.data.unwrap_or_default(),
                    content: markdown_to_html(&result.content),
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

/*
 base16-ocean.dark,base16-eighties.dark,base16-mocha.dark,base16-ocean.light
InspiredGitHub from here
Solarized (dark) and Solarized (light)
 */
pub fn markdown_to_html(source: &str) -> String {
    let adapter = comrak::plugins::syntect::SyntectAdapterBuilder::new()
        .theme("base16-ocean.dark")
        .build();
    let options = comrak::Options::default();
    let mut plugins = comrak::options::Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    comrak::markdown_to_html_with_plugins(source, &options, &plugins)
}

pub const SITE_URL: &str = "https://abhinandh-s.github.io/";
pub const SITE_LANGUAGE: &str = "en-us";
pub const VERSION: &str = "https://jsonfeed.org/version/1.1";
pub const TITLE: &str = "Abhi's Feed";
pub const HOME_PAGE_URL: &str = "https://abhinandh-s.github.io/";
pub const FEED_URL: &str = "https://abhinandh-s.github.io/feed.json";
pub const DESCRIPTION: &str = "Json feed for articles written by Abhinandh S";
pub const ICON: &str = "https://example.org/favicon-timeline-512x512.png";
pub const FAVICON: &str = "https://example.org/favicon-sourcelist-64x64.png";
pub const LANGUAGE: &str = "en-US";

#[derive(Serialize)]
struct JsonFeed {
    version: String,
    language: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    items: Vec<JsonFeedItem>,
}

#[derive(Serialize)]
struct JsonFeedItem {
    id: String,
    url: String,
    title: String,
    content_html: String,
    date_published: String, // ISO 8601 format
    summary: Option<String>,
    banner_image: Option<String>,
}

fn generate_json_feed() -> String {
    let articles = get_all_articles_sorted();
    let items: Vec<JsonFeedItem> = articles
        .into_iter()
        .map(|article| JsonFeedItem {
            id: article.id.clone(),
            url: format!("{}articles/{}", HOME_PAGE_URL, article.id),
            title: article.matter.title,
            content_html: article.content,
            date_published: format_rfc3339(&article.matter.published_at),
            summary: Some(article.matter.snippet),
            banner_image: None,
        })
        .collect();
    let feed = JsonFeed {
        version: VERSION.into(),
        language: LANGUAGE.into(),
        title: TITLE.into(),
        home_page_url: HOME_PAGE_URL.into(),
        feed_url: FEED_URL.into(),
        items,
    };
    serde_json::to_string_pretty(&feed).unwrap_or_default()
}

pub fn format_rfc3339(input: &str) -> String {
    // Try parsing as YYYY-MM-DD HH:MM first
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M") {
        return chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    }

    // Fallback to YYYY-MM-DD and assume midnight UTC
    if let Ok(d) = chrono::NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        let dt = d.and_hms_opt(0, 0, 0).unwrap();
        return chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
            .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    }

    // Return original or a fallback if parsing fails completely
    input.to_string()
}

fn generate_rss_feed() -> String {
    let articles = get_all_articles_sorted();

    let mut items = String::new();

    for article in articles {
        items.push_str(&format!(
            r#"
      <item>
        <title><![CDATA[{title}]]></title>
        <link>{site}articles/{id}</link>
        <guid>{site}articles/{id}</guid>
        <pubDate>{date}</pubDate>
        <description><![CDATA[{summary}]]></description>
        <content:encoded><![CDATA[{content}]]></content:encoded>
      </item>
"#,
            title = article.matter.title,
            id = article.id,
            site = SITE_URL,
            date = format_rfc3339(&article.matter.published_at),
            summary = article.matter.snippet,
            content = article.content
        ));
    }

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0"
  xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>{title}</title>
    <link>{site}</link>
    <description>{desc}</description>
    <language>{lang}</language>
    {items}
  </channel>
</rss>
"#,
        title = TITLE,
        site = SITE_URL,
        desc = DESCRIPTION,
        lang = SITE_LANGUAGE,
        items = items
    )
}

fn generate_atom_feed() -> String {
    let articles = get_all_articles_sorted();

    let updated = articles
        .first()
        .map(|a| format_rfc3339(&a.matter.published_at))
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

    let mut entries = String::new();

    for article in articles {
        entries.push_str(&format!(
            r#"
  <entry>
    <title>{title}</title>
    <link href="{site}articles/{id}"/>
    <id>{site}articles/{id}</id>
    <updated>{updated}</updated>
    <summary>{summary}</summary>
    <content type="html"><![CDATA[{content}]]></content>
  </entry>
"#,
            title = article.matter.title,
            id = article.id,
            site = SITE_URL,
            updated = format_rfc3339(&article.matter.published_at),
            summary = article.matter.snippet,
            content = article.content
        ));
    }

    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>{title}</title>
  <link href="{site}"/>
  <link href="{site}atom.xml" rel="self"/>
  <updated>{updated}</updated>
  <id>{site}</id>
  {entries}
</feed>
"#,
        title = TITLE,
        site = SITE_URL,
        updated = updated,
        entries = entries
    )
}
