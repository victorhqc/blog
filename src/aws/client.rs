use aws_sdk_s3::Client;

pub async fn build_client() -> Result<Client, aws_sdk_s3::Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    Ok(client)
}
