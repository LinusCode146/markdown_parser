use std::process;
use crate::{read_md_file, Config, Markdown};

#[test]
fn test_config_builder() {
    let config = Config::build(vec![String::from("hey"), String::from("HarryP0tter.html"), String::from("hi.md")]
        .into_iter())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    assert_eq!(config.filepath_md, String::from("hi.md"));
    assert_eq!(config.filepath_html, String::from("HarryP0tter.html"));
}

#[test]
#[should_panic(expected = "Invalid HTML filepath")]
fn test_wrong_html_paths() {
    Config::build(vec![String::from("hey"), String::from("HarryP0tter.htl"), String::from("hi.md")]
        .into_iter()).unwrap();
}

#[test]
#[should_panic(expected = "Invalid Markdown filepath")]
fn test_wrong_md_paths() {
    Config::build(vec![String::from("hey"), String::from("HarryP0tter.html"), String::from("hi.m")]
        .into_iter()).unwrap();
}

#[test]
fn test_create_markdown_object() {
    let config = Config::build(vec![String::from("hey"), String::from("index.html"), String::from("hello.md")]
        .into_iter()).unwrap();

    let mk_file = read_md_file(&config).expect("something is wrong with the md file");
    assert_eq!(mk_file, Markdown {
        config: &config,
        lines: vec!["# Überschrift 1".to_string(), "## Überschrift 2".to_string(), "### Überschrift 3".to_string(), "#### Überschrift 4".to_string(), "".to_string(),  "Ganz normaler text".to_string()],
        html: "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n    <title>Hello</title>\n</head>\n<body>\n    <h1>Hi</h1>\n</body>\n</html>".to_string(),
    })
}