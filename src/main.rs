use apod_rust_client::{ApodClient, BaseApodClient};

#[tokio::main]
async fn main() {
    println!("Entering main function");
    let client = BaseApodClient::build("0GGQn6xgIdHnVbNo9eKBI1lZstLXL0304xH0MICL");

    let _ = client.get_latest_apod().await;
    let _ = client.get_apod("2023-04-01").await;
    let _ = client.get_random_apods(5).await;
}
