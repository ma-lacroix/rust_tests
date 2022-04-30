use std::fs::File;
use std::io::{stdout, Write};
use curl::easy::Easy;

pub struct get_request {
    pub url: String,
}

impl get_request {

    pub fn new(url: String) -> Self {
        Self {
            url: url.to_string(),
        }
    }
    pub fn get_data(&self) {
        // let mut file = File::create("data.csv");
        // let mut easy = Easy::new();
        // easy.url(&self.url).unwrap();
        // easy.write_function(|data| {
        //     stdout().write_all(data).unwrap();
        //     Ok(data.len())
        // }).unwrap();
        // easy.perform().unwrap();
        let mut buf = Vec::new();
        let mut handle = Easy::new();
        handle.url("https://www.rust-lang.org/").unwrap();

        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            buf.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
        drop(transfer);
        println!("out: {}", std::from_utf8(&buf).unwrap());
    }
}