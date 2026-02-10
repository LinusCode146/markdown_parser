use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub filepath_html: String,
    pub filepath_md: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let filepath_html = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an html filepath here"),
        };

        if !filepath_html.ends_with(".html") || filepath_html == ".html" {
            return Err("Invalid HTML filepath");
        }

        let filepath_md = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an markdown filepath here"),
        };

        if !filepath_md.ends_with(".md") || filepath_md == ".md" {
            return Err("Invalid Markdown filepath");
        }

        Ok(Config { filepath_html, filepath_md })
    }
}

#[derive(Debug, PartialEq)]
pub struct Markdown<'a> {
    pub config: &'a Config,
    pub lines: Vec<String>,
    pub html: String,
}

impl<'a> Markdown<'a> {
    pub fn info(&self) {
        println!("\n");
        self.lines.iter().for_each(|line| println!("{}", line));
    }

    pub fn write_html_to_file(&self) -> Result<(), Box<dyn Error>> {
        fs::write(&self.config.filepath_html, &self.html)?;

        println!("\n");
        println!("Successfully created index.html file");
        Ok(())
    }

    pub fn convert_md_to_html(&self) {
        println!("Converted markdown file");
    }
}

pub fn read_md_file<'b>(config: &'b Config) -> Result<Markdown<'b>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath_md)?;
    let mk = Markdown {
        lines: contents
            .lines()
            .map(|line| line.to_string())
            .collect(),
        html: generate_html("Hello", "<h1>Hi</h1>"),
        config
    };
    Ok(mk)
}

pub fn generate_html(title: &str, content: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    {}
</body>
</html>"#, title, content)
}