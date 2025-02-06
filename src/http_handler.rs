use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{Body, Error, Request, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    println!("Using region: {:?}", config.region().unwrap());

    let client = Client::new(&config);
    let email_list_table = String::from("NewsLetterSubscribers");

    Ok(Response::builder()
        .status(200)
        .body(Body::from(format!("working...: ")))
        .expect("Failed to construct response"))
}
