mod client;
use client::Client;
use dotenv::dotenv;
use std::env::var;

fn main() {
    dotenv().ok();
    let ip = var("IPV4").expect("Can't find .evn variable IPV4");
    let port = var("PORT").expect("Can't find .evn variable IPV4");
    let client = Client::new(ip, port).expect("Failed to connect to host");
}
