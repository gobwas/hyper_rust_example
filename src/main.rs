extern crate hyper;
extern crate regex;

use std::io::Write;
use regex::{Regex, Captures};

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;
use hyper::uri::RequestUri::{AbsolutePath};

fn handler(req: Request, res: Response<Fresh>) {
    let greeting_re = Regex::new(r"^/greeting/([a-z]+)$").unwrap();

    match req.uri {
        AbsolutePath(ref path) => match (&req.method, &path[..]) {
            (&hyper::Get, "/") => {
                hello(&req, res);
            },
            _ => {
                if greeting_re.is_match(path) {
                    let cap = greeting_re.captures(path).unwrap();
                    greet(&req, res, cap);
                } else {
                    not_found(&req, res);
                }
            }
        },
        _ => {
            not_found(&req, res);
        }
    };
}

fn hello(_: &Request, res: Response<Fresh>) {
    let mut r = res.start().unwrap();
    r.write_all(b"Hello World!").unwrap();
    r.end().unwrap();
}

fn greet(_: &Request, res: Response<Fresh>, cap: Captures) {
    let mut r = res.start().unwrap();
    r.write_all(format!("Hello, {}", cap.at(1).unwrap()).as_bytes()).unwrap();
    r.end().unwrap();
}

fn not_found(_: &Request, mut res: Response<Fresh>) {
    *res.status_mut() = hyper::NotFound;
    let mut r = res.start().unwrap();
    r.write_all(b"Not Found\n").unwrap();
}

fn main() {
    println!("will listen 40000");
    let _ = Server::http("127.0.0.1:4000").unwrap().handle(handler);
}
