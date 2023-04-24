use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};

// const NASA_APOD_ENDPOINT: &str = "https://api.nasa.gov/planetary/apod";

#[derive(Serialize, Deserialize)]
pub struct Apod {
    title: String,
    date: String,
    url: String,
    hdurl: String,
    media_type: String,
    explanation: String,
    copyright: String,
}

impl std::fmt::Display for Apod {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "APOD \n
            title: {} \n
            date: {} \n
            url: {} \n
            hdurl: {} \n
            media_type: {} \n
            explanation: {} \n
            copyright: {}",
            self.title,
            self.date,
            self.url,
            self.hdurl,
            self.media_type,
            self.explanation,
            self.copyright
        )
    }
}

#[async_trait]
pub trait ApodClient {
    fn build(api_key: &str) -> Self;
    async fn get_apod(&self) -> Apod;
}

#[derive(Debug)]
pub struct BaseApodClient {
    pub api_key: String,
}

#[async_trait]
impl ApodClient for BaseApodClient {
    fn build(api_key: &str) -> BaseApodClient {
        if !ApiKeyValidator::is_valid(api_key) {
            panic!("API Key is invalid")
        }

        let api_key: String = api_key.to_string();

        BaseApodClient { api_key }
    }

    async fn get_apod(&self) -> Apod {
        let url = "https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY";
        let apod = reqwest::get(url)
            .await
            .unwrap()
            .json::<Apod>()
            .await
            .unwrap();

        println!("Testing");
        println!("{}", apod);
        return apod;
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
