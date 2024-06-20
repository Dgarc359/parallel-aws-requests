use futures::{stream, StreamExt};

#[tokio::main]
async fn main() {
    let aws_config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let t = stream::iter(0..3)
        .map(|_| {
            let s3_client = &s3_client;
            async move {
                s3_client.list_buckets().send().await
            }
        })
        .buffer_unordered(2usize);

    let _ = t.for_each(|result| async {
        let result = result.unwrap();
        dbg!("response returned for s3");

        result.buckets.iter().for_each(|buckets| {
            buckets.iter().for_each(|bucket| {
                println!("{:#?}",bucket.name().unwrap());
            });
        });
    }).await;

}
