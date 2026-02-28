use std::env;
use std::error::Error;
use std::fs;
use std::time::Duration;

use futures::stream::{self, StreamExt};
use regex::Regex;
use scraper::{Html, Selector};
use reqwest::{Client, StatusCode};

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

const CONCURRENCY_LIMIT: usize = 32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} <input.md> [output.md]", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = args.get(2).map(String::as_str).unwrap_or("output.md");

    let markdown = fs::read_to_string(input_path)?;
    let urls = extract_urls(&markdown);

    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    let mut results = stream::iter(urls.into_iter().enumerate().map(|(idx, url)| {
        let client = client.clone();
        async move { (idx, process_url(client, url).await) }
    }))
    .buffer_unordered(CONCURRENCY_LIMIT)
    .collect::<Vec<_>>()
    .await;

    results.sort_by_key(|(idx, _)| *idx);

    let output = results
        .into_iter()
        .map(|(_, result)| format_output_line(result))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(output_path, output)?;
    Ok(())
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

async fn process_url(client: Client, url: String) -> Result<UrlCheckSuccess, UrlCheckError> {
    let response = match client.get(&url).send().await {
        Ok(response) => response,
        Err(error) => {
            return Err(UrlCheckError {
                url,
                message: reqwest_error_message(&error),
            });
        }
    };

    let status = response.status();
    if !status.is_success() {
        return Err(UrlCheckError {
            url,
            message: format_status(status),
        });
    }

    let body = match response.text().await {
        Ok(body) => body,
        Err(error) => {
            return Err(UrlCheckError {
                url,
                message: reqwest_error_message(&error),
            });
        }
    };

    let title = extract_title(&body).unwrap_or_else(|| "NO TITLE FOUND".to_string());
    Ok(UrlCheckSuccess { url, title })
}

fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("title").expect("valid title selector");
    document
        .select(&selector)
        .next()
        .map(|node| node.text().collect::<String>().trim().to_string())
        .filter(|title| !title.is_empty())
}

fn format_output_line(result: Result<UrlCheckSuccess, UrlCheckError>) -> String {
    match result {
        Ok(success) => {
            format!("[ {} ] ( {} )", sanitize_text(&success.title), success.url)
        }
        Err(error) => {
            format!("[ {} ] ( {} )", sanitize_text(&error.message), error.url)
        }
    }
}

fn sanitize_text(text: &str) -> String {
    text.replace('\n', " ").replace('\r', " ")
}

fn format_status(status: StatusCode) -> String {
    match status.canonical_reason() {
        Some(reason) => format!("HTTP {} {}", status.as_u16(), reason),
        None => format!("HTTP {}", status.as_u16()),
    }
}

fn reqwest_error_message(error: &reqwest::Error) -> String {
    if error.is_timeout() {
        "REQUEST TIMEOUT".to_string()
    } else if error.is_connect() {
        "CONNECTION ERROR".to_string()
    } else if error.is_request() {
        "INVALID REQUEST".to_string()
    } else if error.is_body() {
        "RESPONSE BODY ERROR".to_string()
    } else if error.is_decode() {
        "RESPONSE DECODE ERROR".to_string()
    } else {
        "NETWORK ERROR".to_string()
    }
}