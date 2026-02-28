use std::env;
use std::error::Error;
use regex::Regex;

#[derive(Debug)]
struct UrlCheckSuccess {
    url: String,
    title: String,
}

#[derive(Debug)]
struct UrlCheckError {
    url: String,
    message: String,
}

fn extract_urls(markdown: &str) -> Vec<String> {
    let url_re = Regex::new(
        r"\[[^\]]*\]\((?P<md>https?://[^\s)]+)\)|<(?P<auto>https?://[^>\s]+)>|(?P<bare>https?://[^\s)>\]]+)",
    )
    .expect("valid URL extraction regex");

    let mut urls = Vec::new();

    for captures in url_re.captures_iter(markdown) {
        if let Some(url) = captures
            .name("md")
            .or_else(|| captures.name("auto"))
            .or_else(|| captures.name("bare"))
        {
            urls.push(url.as_str().to_string());
        }
    }

    urls
}