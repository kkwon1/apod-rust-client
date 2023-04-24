use apod_rust_client::{ApodClient, BaseApodClient};

#[tokio::main]
async fn main() {
    println!("Entering main function");
    let client = BaseApodClient {
        api_key: String::from("someKey"),
    };

    let _ = client.get_apod().await;
}
