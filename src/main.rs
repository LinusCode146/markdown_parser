use std::{env, fs, process};
use std::error::Error;
use markdown_parser::{Config, Markdown};


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    print!("Html Filepath: {:?}", config.filepath_html);
    print!("Markdown Filepath: {:?}", config.filepath_md);

    let mk_file = read_md_file(&config).expect("something is wrong with the md file");
    mk_file.info();
    mk_file.write_html_to_file().expect("something went wrong creating the html file");
}


fn read_md_file<'b>(config: &'b Config) -> Result<Markdown<'b>, Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filepath_md)?;
    let mk = Markdown {
        lines: contents
            .lines()
            .map(|line| line.to_string())
            .collect(),
        html: generate_html("Hallooo", "<h1>Bonjouuuuur</h1>"),
        config
    };
    Ok(mk)
}

fn generate_html(title: &str, content: &str) -> String {
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