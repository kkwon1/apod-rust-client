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

#[tokio::test]
async fn test_get_apod_in_date_range() {
    let api_key = get_api_key();
    let client: ApodClient = ApodClient::build(&api_key).unwrap();
    let apods = client.get_apod_from_to("2023-04-01", "2023-04-05").await;

    assert_eq!(apods.len(), 5);

    let first_apod = apods.get(0).unwrap();
    assert_eq!(first_apod.date, "2023-04-01");
    assert_eq!(first_apod.media_type, "image");
    assert_eq!(first_apod.title, "NGC 2442: Galaxy in Volans");
    assert_eq!(
        first_apod.url,
        "https://apod.nasa.gov/apod/image/2304/NGC2442-NicolasROLLAND_signatur1024.jpg"
    );
    assert!(first_apod.explanation.starts_with("Distorted galaxy NGC 2442 can be found in the southern constellation of the flying fish, (Piscis) Volans."));

    let last_apod = apods.get(4).unwrap();
    assert_eq!(last_apod.date, "2023-04-05");
    assert_eq!(last_apod.media_type, "image");
    assert_eq!(last_apod.title, "Rubin's Galaxy");
    assert_eq!(
        last_apod.url,
        "https://apod.nasa.gov/apod/image/2304/RubinsGalaxy_hst1024.jpg"
    );
    assert!(last_apod.explanation.starts_with("In this Hubble Space Telescope image the bright, spiky stars lie in the foreground toward the heroic northern constellation Perseus and well within our own Milky Way galaxy."));
}

#[tokio::test]
async fn test_invalid_api_key() {
    let invalid_api_key: &str = "this_is_invalid";
    let res = ApodClient::build(invalid_api_key);
    assert!(res.is_err());
}
