use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{Body, Error, Request, Response};
use std::collections::HashMap;

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

    let raw_items = get_all_items(&client, &email_list_table).await?;

    println!("raw_items {:?} ", raw_items);

    Ok(Response::builder()
        .status(200)
        .body(Body::from("working...: "))
        .expect("Failed to construct response"))
}

async fn get_all_items(client: &Client, table_name: &String) -> Result<Vec<HashMap<String, AttributeValue>>, aws_sdk_dynamodb::Error> {
    let mut items = Vec::new();
    let mut last_evaluated_key = None;

    loop {
        let resp = client
            .scan()
            .table_name(table_name)
            .set_exclusive_start_key(last_evaluated_key)
            .send()
            .await?;

        if let Some(new_items) = resp.items {
            items.extend(new_items);
        }

        last_evaluated_key = resp.last_evaluated_key;

        if last_evaluated_key.is_none() {
            break;
        }
    }

    Ok(items)

} 

