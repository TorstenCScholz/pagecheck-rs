extern crate reqwest;
extern crate select;
extern crate clap;

use select::document::{Document};
use select::predicate::{Class};

use clap::{App, Arg};

use std::io::Read;

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!("Usage: {} FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }

// TODO: Use error type as return value
fn get_webpage(url: &str) -> Document {
    let mut response = reqwest::get(url).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    Document::from(&body[..])
}

fn main() {
    // TODO: Crawl every X seconds with a maximum deviation of Y seconds

    let matches = App::new("pagecrawl")
                        .version("0.1")
                        .author("Torsten Scholz")
                        .about("GETs a webpage and tests whether a specific class is present.")
                        .args(&[
                            Arg::with_name("url")
                                .help("the url of the HTML document to be tested")
                                .index(1)
                                .required(true),
                            Arg::with_name("classname")
                                .help("the class that should be found within the document")
                                .index(2)
                                .required(true)
                            ])
                        .get_matches();

    let url = matches.value_of("url").unwrap();
    let class_name = matches.value_of("classname").unwrap();

    let document = get_webpage(&url[..]);

    if let Some(quantitiy_field) = document.find(Class(class_name)).next() {
        // println!("Field: {:?}", quantitiy_field);
        println!("Found!");

        // TODO: Tweet about it
    } else {
        println!("Not found!");

        // TODO: Tweet about is
    }
}
