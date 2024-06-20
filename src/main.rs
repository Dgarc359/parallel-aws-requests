use futures::{stream, StreamExt};
use rs_file_utils;

#[tokio::main]
async fn main() {
    let folder_name = "out";
    let files = rs_file_utils::get_filenames_from_folder(folder_name).unwrap();
    dbg!(files);

    // let's say hypothetically you want to upload files to 3 buckets in one go..
    // the exercise of iterating through the file names, retrieving their contents
    // and uploading to S3, is left to the reader. This example serves to show
    // how one can begin to send requests to AWS in parallel

    let aws_config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let t = stream::iter(0..3)
        .map(|_| {
            let s3_client = s3_client.clone();
            tokio::spawn(async move {
                s3_client.list_buckets().send().await
            })
        })
        // NOTE: this determines the amount of concurrent requests possible
        .buffer_unordered(2usize);

    let _ = t.for_each(|result| async {
        let result = result.unwrap();
        dbg!("response returned for s3");
        match result {
            Ok(list_buckets_output) => {
                list_buckets_output.buckets().iter().for_each(|bucket| {
                    println!("{}", bucket.name().unwrap());
                });
            },
            Err(e) => eprintln!("error grabbign buckets {}", e),
        }
    }).await;

}
