use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{self, service_fn, Error, LambdaEvent};

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<(), Error> {
    let (event, _) = event.into_parts();

    for record in event.records {
        println!("Record: {:?}", record);

        let bucket = record.s3.bucket;
        let object = record.s3.object;

        println!("Bucket: {:?}", bucket);
        println!("Object: {:?}", object);
    }

    // Process the file and split it as needed, potentially invoking other Lambda functions

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(function_handler)).await?;
    Ok(())
}
