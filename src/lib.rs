#[cfg(test)]
mod tests;

#[derive(Debug)]
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