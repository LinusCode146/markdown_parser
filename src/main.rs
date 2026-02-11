use std::{env, process};
use markdown_parser::{Config, read_md_file};


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut mk_file = read_md_file(config).expect("something is wrong with the md file");
    mk_file.info();
    mk_file.convert_md_to_html();
    mk_file.write_html_to_file().expect("something went wrong creating the html file");
}


