use async_trait::async_trait;
use regex::Regex;

const NASA_APOD_ENDPOINT: &str = "https://api.nasa.gov/planetary/apod";

pub struct Apod {
    title: String,
    date: String,
    url: String,
    hdurl: String,
    media_type: String,
    explanation: String,
    thumbnail_url: String,
    copyright: String,
}

#[async_trait]
pub trait ApodClient {
    fn build(api_key: &str) -> Self;
    fn get_apod(date: &str) -> Apod;
}

#[derive(Debug)]
pub struct BaseApodClient {
    api_key: String,
}

impl ApodClient for BaseApodClient {
    fn build(api_key: &str) -> BaseApodClient {
        if (!ApiKeyValidator::is_valid(api_key)) {
            panic!("API Key is invalid")
        }

        let api_key: String = api_key.to_string();

        BaseApodClient { api_key }
    }

    fn get_apod(date: &str) {
        let url = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";
        let result = reqwest::get(url);
        println!("{:?}", result);
    }
}

struct ApiKeyValidator {}

impl ApiKeyValidator {
    fn is_valid(api_key: &str) -> bool {
        // The valid regex for a NASA API key consists of lower, upper case alphabet, and digits.
        // It must be exactly 40 characters long.
        let regex: Regex = Regex::new(r"^[a-zA-Z0-9]{40}$").unwrap();

        return regex.is_match(api_key);
    }
}
