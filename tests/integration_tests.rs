use apod_rust_client::ApodClient;
use dotenv::dotenv;
use std::env;

fn get_api_key() -> String {
    dotenv().ok();
    env::var("NASA_API_KEY").unwrap()
}

#[tokio::test]
async fn test_get_apod() {
    let api_key = get_api_key();
    let client: ApodClient = ApodClient::build(&api_key).unwrap();
    let apod = client.get_apod("2023-04-01").await;

    assert_eq!(apod.date, "2023-04-01");
    assert_eq!(apod.media_type, "image");
    assert_eq!(apod.title, "NGC 2442: Galaxy in Volans");
    assert_eq!(
        apod.url,
        "https://apod.nasa.gov/apod/image/2304/NGC2442-NicolasROLLAND_signatur1024.jpg"
    );
    assert!(apod.explanation.starts_with("Distorted galaxy NGC 2442 can be found in the southern constellation of the flying fish, (Piscis) Volans."));
}

#[tokio::test]
async fn test_get_random_apods() {
    let api_key = get_api_key();
    let client: ApodClient = ApodClient::build(&api_key).unwrap();
    let apods = client.get_random_apods(3).await;

    assert_eq!(apods.len(), 3);
}
