pub trait ApodClient {
    fn get_config(&self) -> &Config;
}
