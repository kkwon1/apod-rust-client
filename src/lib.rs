use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Apod {
    title: String,
    date: String,
    url: String,
    hdurl: String,
    media_type: String,
    explanation: String,
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
            explanation: {}",
            self.title, self.date, self.url, self.hdurl, self.media_type, self.explanation,
        )
    }
}

#[async_trait]
pub trait ApodClient {
    fn build(api_key: &str) -> Self;
    async fn get_latest_apod(&self) -> Apod;
    async fn get_apod(&self, date: &str) -> Apod;
    async fn get_random_apods(&self, count: i8) -> Vec<Apod>;
}

#[derive(Debug)]
pub struct BaseApodClient {
    pub api_key: String,
}

#[async_trait]
impl ApodClient for BaseApodClient {
    // Constructor for BaseApodClient struct.
    // It validates that the API Key is well formed, and then creates the struct
    fn build(api_key: &str) -> BaseApodClient {
        if !ApiKeyValidator::is_valid(api_key) {
            panic!("API Key is invalid")
        }

        let api_key: String = api_key.to_string();

        BaseApodClient { api_key }
    }

    // The simplest API call is to make the request with no additional parameters
    // besides the API Key. This results in returning the latest APOD
    async fn get_latest_apod(&self) -> Apod {
        let url = build_url(self);
        let apod = reqwest::get(url)
            .await
            .unwrap()
            .json::<Apod>()
            .await
            .unwrap();

        println!("{}", apod);
        return apod;
    }

    // Return the APOD for a specified date.
    // The format for date must always be `yyyy-mm-dd`
    async fn get_apod(&self, date: &str) -> Apod {
        let url = build_url(self);
        let date_url = format!("{}{}{}", url, "&date=", date);
        let apod = reqwest::get(date_url)
            .await
            .unwrap()
            .json::<Apod>()
            .await
            .unwrap();

        println!("{}", apod);
        return apod;
    }

    async fn get_random_apods(&self, count: i8) -> Vec<Apod> {
        let url = build_url(self);
        let date_url = format!("{}{}{}", url, "&count=", count);
        let apods = reqwest::get(date_url)
            .await
            .unwrap()
            .json::<Vec<Apod>>()
            .await
            .unwrap();

        for apod in &apods {
            println!("{}", apod);
        }

        return apods;
    }
}

fn build_url(client: &BaseApodClient) -> String {
    let base_url = "https://api.nasa.gov/planetary/apod?api_key=".to_string();
    format!("{}{}", base_url, client.api_key)
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
