#![recursion_limit = "1024"]

extern crate reqwest;
extern crate select;
extern crate clap;
extern crate lettre;
extern crate dotenv;

#[macro_use]
extern crate error_chain;

use select::document::{Document};
use select::predicate::{Class};

use clap::{App, Arg};

use lettre::email::EmailBuilder;
use lettre::transport::EmailTransport;
use lettre::transport::smtp::{SmtpTransportBuilder, SMTP_PORT};

use std::io::Read;

mod errors {
    error_chain! { }
}

use errors::*;

const EMAIL_USERNAME: &'static str = "EMAIL_USERNAME";
const EMAIL_PASSWORD: &'static str = "EMAIL_PASSWORD";
const EMAIL_TO: &'static str = "EMAIL_TO";
const EMAIL_TO_FULLNAME: &'static str = "EMAIL_TO_FULLNAME";
const EMAIL_FROM: &'static str = "EMAIL_FROM";
const EMAIL_FROM_FULLNAME: &'static str = "EMAIL_FROM_FULLNAME";
const EMAIL_SERVER_ADDRESS: &'static str = "EMAIL_SERVER_ADDRESS";

// TODO: Use error type as return value
fn get_webpage(url: &str) -> Document {
    let mut response = reqwest::get(url).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    Document::from(&body[..])
}

fn run() {
    // TODO: Crawl every X seconds with a maximum deviation of Y seconds

    dotenv::dotenv().ok();

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

    if let Some(_) = document.find(Class(class_name)).next() {
        println!("Found!");

        send_email("Product available. Buy now!", "The watched product seems to be available. Buy now before it is too late.");
    } else {
        println!("Not found!");

        send_email("Product not available", "The watched product is not available.");
    }
}

fn send_email(subject: &str, content: &str) {
    let email_username = std::env::var(EMAIL_USERNAME).unwrap();
    let email_password = std::env::var(EMAIL_PASSWORD).unwrap();
    let email_to = std::env::var(EMAIL_TO).unwrap();
    let email_to_fullname = std::env::var(EMAIL_TO_FULLNAME).unwrap();
    let email_from = std::env::var(EMAIL_FROM).unwrap();
    let email_from_fullname = std::env::var(EMAIL_FROM_FULLNAME).unwrap();
    let email_server_address = std::env::var(EMAIL_SERVER_ADDRESS).unwrap();

    let email = EmailBuilder::new()
        .to((email_to.as_str(), email_to_fullname.as_str()))
        .from((email_from.as_str(), email_from_fullname.as_str()))
        .subject(subject)
        .text(content)
        .build()
        .unwrap();

    let mut mailer = SmtpTransportBuilder::new((email_server_address.as_str(), SMTP_PORT))
        .unwrap()
        .credentials(email_username.as_str(), email_password.as_str())
        .build();

    let result = mailer.send(email);

    if !result.is_ok() {
        println!("Could not send email: {:?}", result);
    }
}

fn main() {
    run();
    // if let Err(ref e) = run() {
    //     println!("error: {}", e);
    //
    //     for e in e.iter().skip(1) {
    //         println!("caused by: {}", e);
    //     }
    //
    //     if let Some(backtrace) = e.backtrace() {
    //         println!("backtrace: {:?}", backtrace);
    //     }
    //
    //     std::process::exit(1);
    // }
}
