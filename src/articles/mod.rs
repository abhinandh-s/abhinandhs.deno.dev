#![allow(unused)]

use std::fs::read_to_string;

use chrono::Datelike;
use pulldown_cmark::{Parser, html};
use serde::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};

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

#[allow(clippy::vec_init_then_push)]
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


    articles.push(Article {
        id: "post".into(),
        matter: FrontMatter {
            title: "This is title".into(),
            published_at: "2026-01-13".into(),
            snippet: "snippet for post".into(),
            tags: None,
        },
        content: format!("This is the content:\n dbg:\n{}\n\n", dbg),
    });
    articles
}

fn is_markdown(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".md"))
        .unwrap_or(false)
}

// input: `2026-01-12 21:34`
// return it as [`Monday, November 25, 2024`]
pub fn date_time(published_at: &str) -> String {
    chrono::DateTime::parse_from_str(published_at, "%Y %b %d")
        .map(|date_time| {
            let s = match date_time.month() {
                1 => "Jan",
                2 => "Feb",
                3 => "Mar",
                4 => "Apr",
                5 => "May",
                6 => "Jun",
                7 => "Jul",
                8 => "Aug",
                9 => "Sep",
                10 => "Oct",
                11 => "Nov",
                12 => "Dec",
                _ => "Unreachable arm",
            };
            format!(
                "{}, {} {}, {}",
                date_time.weekday(),
                s,
                date_time.day(),
                date_time.year()
            )
        })
        .unwrap_or_default()
}

pub fn get_article_by_id(id: &str) -> Option<Article> {
    get_all_articles().into_iter().find(|f| f.id == id)
}

pub fn markdown_to_html(md: &str) -> String {
    let options = pulldown_cmark::Options::all();
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
