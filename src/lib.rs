use regex::Regex;
use reqwest::Error;
use serde::{de, Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Apod {
    title: String,
    date: String,
    url: String,
    hdurl: String,
    media_type: String,
    explanation: String,
    thumbnail_url: Option<String>,
    copyright: Option<String>,
}

pub struct ApodClient {
    api_key: String,
}

impl ApodClient {
    // Constructor for ApodClient struct.
    // It validates that the API Key is well formed, and then creates the struct
    pub fn build(api_key: &str) -> ApodClient {
        if !ApiKeyValidator::is_valid(api_key) {
            panic!("API Key is invalid")
        }

        let api_key: String = api_key.to_string();

        ApodClient { api_key }
    }

    // The simplest API call is to make the request with no additional parameters
    // besides the API Key. This results in returning the latest APOD
    pub async fn get_latest_apod(&self) -> Apod {
        let url = build_url(self);
        get_apod(&url).await.unwrap()
    }

    // Return the APOD for a specified date.
    // The format for date must always be `yyyy-mm-dd`
    pub async fn get_apod(&self, date: &str) -> Apod {
        let url = build_url(self);
        let date_url = format!("{}{}{}", url, "&date=", date);
        get_apod(&date_url).await.unwrap()
    }

    // Return a vector of APODs given a count
    // The APODs are selected at random.
    // `count` input must be greater than 0 and less than or equal to 100
    pub async fn get_random_apods(&self, count: u32) -> Vec<Apod> {
        let url = build_url(self);
        let date_url = format!("{}{}{}", url, "&count=", count);
        get_apod(&date_url).await.unwrap()
    }

    pub async fn get_apod_from(&self, start_date: &str) -> Vec<Apod> {
        let url = build_url(self);
        let date_url = format!("{}{}{}", url, "&start_date=", start_date);
        get_apod(&date_url).await.unwrap()
    }
    pub async fn get_apod_from_to(&self, start_date: &str, end_date: &str) -> Vec<Apod> {
        let url = build_url(self);
        let date_url = format!(
            "{}{}{}{}{}",
            url, "&start_date=", start_date, "&end_date=", end_date
        );
        get_apod(&date_url).await.unwrap()
    }
}

fn build_url(client: &ApodClient) -> String {
    let base_url: String = "https://api.nasa.gov/planetary/apod?api_key=".to_string();
    format!("{}{}", base_url, client.api_key)
}

async fn get_apod<T: de::DeserializeOwned>(url: &str) -> Result<T, Error> {
    let response = reqwest::get(url).await;
    let apods = match response {
        Ok(response) => response.json::<T>().await,
        Err(e) => {
            println!("Failed to receive response: {}", e);
            Err(e)
        }
    };
    match apods {
        Ok(apods) => Ok(apods),
        Err(e) => {
            println!("Failed to parse as APOD: {}", e);
            Err(e)
        }
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

#[cfg(test)]
mod tests {
    use crate::ApiKeyValidator;

    #[test]
    fn valid_api_key() {
        let is_valid = ApiKeyValidator::is_valid("hrDwl56I9DKfPstNy9cqaTn0S68dTYpo4kB96dku");
        assert_eq!(is_valid, true);
    }

    #[test]
    fn invalid_api_key_too_short() {
        let is_valid = ApiKeyValidator::is_valid("hrDwl56I9DKfPstNy9cq");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn invalid_api_key_too_long() {
        let is_valid =
            ApiKeyValidator::is_valid("hrDwl56I9DKfPstNy9cqaTn0S68dTYpo4kB96dkuHfo389sJWE");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn invalid_api_key_special_characters() {
        let is_valid = ApiKeyValidator::is_valid("hrDwl56I9DKfPstNy9cqaTn%S68dTYpo4kB96dku");
        assert_eq!(is_valid, false);
    }
}
