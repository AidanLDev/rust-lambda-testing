use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client, Error as Dynamo_Error};
use aws_sdk_ses::{
    operation::send_email::SendEmailOutput,
    types::{Body as Email_Body, Content, Destination, Message},
    Client as Ses_Client, Error as Ses_Error,
};
use lambda_http::{Body, Error as Lambda_Error, Request, Response};
use std::collections::HashMap;
use uuid::Uuid;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Lambda_Error> {
    // Extract some useful information from the request
    let _region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region("eu-west-2") // Hard coding this to use eu-west-2
        .load()
        .await;

    println!("Using region: {:?}", config.region().unwrap());

    let client = Client::new(&config);
    let ses_client = Ses_Client::new(&config);

    let email_list_table = String::from("Emails");

    add_email(&client, String::from("dev@aidanlowson.com")).await?;
    add_email(&client, String::from("aidanlowson@hotmail.co.uk")).await?;

    let raw_items = get_all_items(&client, &email_list_table).await?;

    let emails: Vec<String> = raw_items
        .iter()
        .filter_map(|item| {
            item.get("subscribed")
                .and_then(|subbed_object| subbed_object.as_bool().ok())
                .map(|subbed| *subbed)
                .filter(|subbed| *subbed)
                .and_then(|_| {
                    item.get("email")
                        .and_then(|email_object| match email_object.as_s() {
                            Ok(s) => Some(s.to_string()),
                            Err(_) => None,
                        })
                })
        })
        .collect();

    println!("Emails: {:?}", emails);

    println!("About to send mail!");

    send_email(&ses_client, vec![String::from("dev@aidanlowson.com")]).await?;

    println!("Emails sent!");

    Ok(Response::builder()
        .status(200)
        .body(Body::from("working...: "))
        .expect("Failed to construct response"))
}

async fn add_email(
    client: &Client,
    email: String,
) -> Result<aws_sdk_dynamodb::operation::put_item::PutItemOutput, Dynamo_Error> {
    let id_av = AttributeValue::S(Uuid::new_v4().to_string());
    let email_av = AttributeValue::S(email);
    let subscribed_av = AttributeValue::Bool(true);

    let req = client
        .put_item()
        .table_name("Emails")
        .item("id", id_av)
        .item("email", email_av)
        .item("subscribed", subscribed_av);

    println!("Executing request [{req:?}] to add an item...");

    let res = req.send().await?;

    println!("Added email!");

    Ok(res)
}

async fn get_all_items(
    client: &Client,
    table_name: &String,
) -> Result<Vec<HashMap<String, AttributeValue>>, Dynamo_Error> {
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

async fn send_email(
    ses_client: &Ses_Client,
    recipients: Vec<String>,
) -> Result<SendEmailOutput, Ses_Error> {
    let sender = "dev@aidanlowson.com";
    let subject = String::from("Hello World");
    let body_text = String::from("Hello World!");
    let body_html = String::from("<html><body><h1>Hello World!</h1></body></html>");

    let destination = Destination::builder()
        .set_bcc_addresses(Some(recipients))
        .build();

    let send_email_builder = ses_client
        .send_email()
        .destination(destination)
        .message(
            Message::builder()
                .subject(Content::builder().data(subject).build()?)
                .body(
                    Email_Body::builder()
                        .text(Content::builder().data(body_text).build()?)
                        .html(Content::builder().data(body_html).build()?)
                        .build(),
                )
                .build(),
        )
        .source(sender);

    let response = send_email_builder.send().await?;

    Ok(response)
}
