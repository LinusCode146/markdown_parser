use std::{env, process};
use markdown_parser::Config;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    print!("Html Filepath: {:?}", config.filepath_html);
    print!("Markdown Filepath: {:?}", config.filepath_md);
}


