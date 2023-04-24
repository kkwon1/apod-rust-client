// Library crates don't have main.rs - we can remove this after we are done experimenting

use regex::Regex;
use reqwest;
use std::error::Error;

const NASA_APOD_ENDPOINT: &str = "https://api.nasa.gov/planetary/apod";
// 40 characters, mix of numbers, lower and uppder case letters
const EXAMPLE_API_KEY: &str = "0GGQn6xgIdHnVbNo9eKBI1lZstLXL0304xH0MICL";

fn main() {
    // chaining .await will yield our query result
    let result = reqwest::get("https://api.spotify.com/v1/search");
    println!("{:?}", result);
}
