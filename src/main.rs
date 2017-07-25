extern crate curl;

use std::io::{stdout, Write};
use std::env;
use curl::easy::Easy;

struct Fetch {}

impl Fetch {
    fn meta(&self, msg: &str) {
        let mut easy = Easy::new();
        let mut foo = "http://registry.npmjs.org/".to_string();
        let baz = "/latest".to_string();

        // & is key
        foo.push_str(&msg);
        foo.push_str(&baz);

        easy.url(&foo).unwrap();
        easy.write_function(|data| {
            Ok(stdout().write(data).unwrap())
        }).unwrap();
        easy.perform().unwrap();
        println!("{}", easy.response_code().unwrap());
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let fetch = Fetch{};
    fetch.meta(&args[1]);
}
