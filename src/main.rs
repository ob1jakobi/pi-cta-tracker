use log::info;
use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    static ref CTA_API_KEY: String = std::env::var("CTA_API_KEY").expect("CTA_API_KEY must be set");
    static ref CTA_BASE_URL: &'static str = "lapi.transitchicago.com/api/1.0/ttarrivals.aspx";
}


#[tokio::main]

/// Loosely based on the following project https://www.youtube.com/watch?v=5bnAVNNSK0w.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();


    // TODO: instantiate an HTTP request client
    info!("Instantiating reqwest client");
    let client = reqwest::Client::new();

    // TODO: authenticate with the API
    info!("Authenticating with CTA API");

    // TODO: Call to get latest info


    Ok(())
}
