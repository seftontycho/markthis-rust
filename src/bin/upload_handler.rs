use std::env;

use chrono::Utc;
use lambda_http::{service_fn, Error, Request};

use rusoto_core::{credential::AwsCredentials, Region};
use rusoto_credential::{ChainProvider, ProvideAwsCredentials};
use rusoto_s3::{
    util::{PreSignedRequest, PreSignedRequestOption},
    GetObjectRequest,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
struct RequestEvent {
    pub file_name: String,
}

#[derive(Serialize)]
struct Response {
    status_code: i32,
    presigned_url: String,
}

async fn function_handler(
    event: Request,
    region: &Region,
    bucket_name: &str,
    credentials: &AwsCredentials,
) -> Result<Value, Error> {
    let event: RequestEvent = serde_json::from_slice(event.body().as_ref())?;

    let presigned_url =
        generate_presigned_url(region, credentials, bucket_name, &event.file_name).await?;

    Ok(json!(Response {
        status_code: 200,
        presigned_url,
    }))
}

async fn generate_presigned_url(
    region: &Region,
    credentials: &AwsCredentials,
    bucket_name: &str,
    file_name: &str,
) -> Result<String, Error> {
    let options = PreSignedRequestOption {
        expires_in: std::time::Duration::from_secs(600),
    };

    let date = Utc::now().to_string();
    let key = format!("{}-{}", file_name, date);

    let get_object_req = GetObjectRequest {
        bucket: bucket_name.to_string(),
        key,
        ..Default::default()
    };

    let url = get_object_req.get_presigned_url(&region, &credentials, &options);

    Ok(url)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region = Region::EuWest2;
    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not set");
    let provider = ChainProvider::new();
    let credentials = provider.credentials().await.unwrap();

    lambda_http::run(service_fn(|event| {
        function_handler(event, &region, &bucket_name, &credentials)
    }))
    .await?;
    Ok(())
}
