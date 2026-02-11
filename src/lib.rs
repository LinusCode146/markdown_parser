use std::error::Error;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests;


lazy_static! {
        static ref IMAGE: Regex = Regex::new(r"!\[([^\]]*)\]\(([^\)]+)\)").unwrap();
        static ref LINK: Regex = Regex::new(r"\[([^\]]+)\]\(([^\)]+)\)").unwrap();
        static ref BOLD_ITALIC: Regex = Regex::new(r"\*\*\*(.+?)\*\*\*").unwrap();
        static ref BOLD_STARS: Regex = Regex::new(r"\*\*(.+?)\*\*").unwrap();
        static ref BOLD_UNDER: Regex = Regex::new(r"__(.+?)__").unwrap();
        static ref ITALIC_STARS: Regex = Regex::new(r"\*(.+?)\*").unwrap();
        static ref ITALIC_UNDER: Regex = Regex::new(r"_(.+?)_").unwrap();
        static ref INLINE_CODE: Regex = Regex::new(r"`(.+?)`").unwrap();
    }


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
pub struct Markdown {
    pub config: Config,
    pub lines: Vec<String>,
    pub html: String,
}

impl Markdown {
    pub fn info(&self) {
        println!("\n");
        self.lines.iter().for_each(|line| println!("{}", line));
    }

    pub fn write_html_to_file(&self) -> Result<(), Box<dyn Error>> {
        fs::write(&self.config.filepath_html, generate_html("Probe", &self.html))?;

        println!("\n");
        println!("Successfully created index.html file");
        Ok(())
    }

    pub fn convert_md_to_html(&mut self) {
        assert!(self.lines.len() > 0);

        self.html = self.lines.iter().map(|x| {
            match x.as_str() {
                line if line.starts_with("#####") => format!("<h5>{}</h5>", self.process_inline_formatting(line.trim_start_matches("#####").trim())),
                line if line.starts_with("####") => format!("<h4>{}</h4>", self.process_inline_formatting(line.trim_start_matches("####").trim())),
                line if line.starts_with("###") => format!("<h3>{}</h3>", self.process_inline_formatting(line.trim_start_matches("###").trim())),
                line if line.starts_with("##") => format!("<h2>{}</h2>", self.process_inline_formatting(line.trim_start_matches("##").trim())),
                line if line.starts_with("#") => format!("<h1>{}</h1>", self.process_inline_formatting(line.trim_start_matches("#").trim())),
                _ => format!("<p>{}</p>", self.process_inline_formatting(x)),
            }
        }).collect::<Vec<String>>().join("\n");
    }



    fn process_inline_formatting(&self, text: &str) -> String {
        let mut result = text.to_string();

        result = IMAGE.replace_all(&result, r#"<img src="$2" alt="$1">"#).to_string();
        result = LINK.replace_all(&result, r#"<a href="$2">$1</a>"#).to_string();
        result = BOLD_ITALIC.replace_all(&result, "<strong><em>$1</em></strong>").to_string();
        result = BOLD_STARS.replace_all(&result, "<strong>$1</strong>").to_string();
        result = BOLD_UNDER.replace_all(&result, "<strong>$1</strong>").to_string();
        result = ITALIC_STARS.replace_all(&result, "<em>$1</em>").to_string();
        result = ITALIC_UNDER.replace_all(&result, "<em>$1</em>").to_string();
        result = INLINE_CODE.replace_all(&result, "<code>$1</code>").to_string();

        result
    }
}

pub fn read_md_file(config: Config) -> Result<Markdown, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath_md)?;
    let mk = Markdown {
        lines: contents
            .lines()
            .map(|line| line.to_string())
            .collect(),
        html: "".to_string(),
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