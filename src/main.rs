use aws_config;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_iam::{Client, Error, Region};
use aws_smithy_types::date_time::Format;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let response = client.list_users().set_max_items(Some(100)).send().await?;
    println!("name|arn|passlastused");
    for user in response.users().unwrap_or_default() {
        let plu = match user.password_last_used() {
            Some(d) => d.fmt(Format::DateTime).unwrap(),
            _ => "".to_string(),
        };
        println!(
            "{}|{}|{}",
            user.user_name().unwrap_or_default(),
            user.arn().unwrap_or_default(),
            plu
        );
    }
    Ok(())
}
