use aws_sdk_s3::{
    output::{DeleteObjectOutput, GetObjectOutput, PutObjectOutput},
    types::ByteStream,
    Client, Error,
};

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    file_data: Vec<u8>,
    key: &str,
    content_type: Option<String>,
) -> Result<PutObjectOutput, Error> {
    let body = ByteStream::from(file_data);
    let response = client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body)
        .set_content_type(content_type)
        .send()
        .await?;

    debug!("Uploaded file to S3: {}", file_name);
    Ok(response)
}

pub async fn download_object(
    client: &Client,
    bucket_name: &str,
    key: &str,
) -> Result<GetObjectOutput, Error> {
    let response = client
        .get_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;

    Ok(response)
}

pub async fn remove_object(
    client: &Client,
    bucket_name: &str,
    key: &str,
) -> Result<DeleteObjectOutput, Error> {
    let response = client
        .delete_object()
        .bucket(bucket_name)
        .key(key)
        .send()
        .await?;

    Ok(response)
}
