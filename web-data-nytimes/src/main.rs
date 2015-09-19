extern crate hyper;

use hyper::Client;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::env;

fn get_data(api_key: &str, category: &str, file_name: &str) {
    let mut file_output = String::new();

    for page in 0..5 {
        let url = format!("http://api.nytimes.com/svc/search/v2/articlesearch.json?fq=news_desk:(%22{category}%22)&page={page}&api-key={api_key}",
                           category = category,
                           page = page,
                           api_key = api_key);

        let client = Client::new();
        let mut res = client.get(&url).send().unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        file_output.push_str(&body);
        file_output.push_str("\n");

        thread::sleep_ms(500); // Throttle API calls
    }

    let mut f = File::create(format!("{}.txt", file_name)).unwrap();
    f.write_all(&file_output.into_bytes()).unwrap();

    println!("{} data written to {}.txt", category, file_name);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("First argument should be API KEY");
        return;
    }

    let api_key = &args[1];

    get_data(api_key, "Arts", "data/arts");
    get_data(api_key, "Sports", "data/sports");

    println!("Done");
}
