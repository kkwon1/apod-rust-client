use apod_rust_client::ApodClient;

#[tokio::main]
async fn main() {
    let client: ApodClient = ApodClient::build("0GGQn6xgIdHnVbNo9eKBI1lZstLXL0304xH0MICL").unwrap();

    let _ = client.get_latest_apod().await;
    let _ = client.get_apod("2023-04-01").await;
    let _ = client.get_random_apods(5).await;
    let _ = client.get_apod_from("2023-05-01").await;
    let _ = client.get_apod_from_to("2023-01-01", "2023-01-05").await;
}
