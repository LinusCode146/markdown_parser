use std::process;
use crate::Config;

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