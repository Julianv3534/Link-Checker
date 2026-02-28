use std::env;
use std::error::Error;

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
