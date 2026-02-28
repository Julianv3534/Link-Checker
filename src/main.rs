use std::env;
use std::error::Error;
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