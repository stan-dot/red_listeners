use futures::StreamExt;
use rabbitmq_stream_client::error::{ConsumerCreateError, StreamCreateError};
use rabbitmq_stream_client::types::{ByteCapacity, OffsetSpecification, ResponseCode};
use std::io::stdin;
use tokio::task;

use rabbitmq_stream_client::Environment;

async fn receive() -> Result<(), ConsumerCreateError> {
    let environment = Environment::builder().build().await?;
    let stream = "hello-rust-stream";
    let create_response = environment
        .stream_creator()
        .max_length(ByteCapacity::GB(5))
        .create(stream)
        .await;

    if let Err(e) = create_response {
        if let StreamCreateError::Create { stream, status } = e {
            match status {
                // we can ignore this error because the stream already exists
                ResponseCode::StreamAlreadyExists => {}
                err => {
                    println!("Error creating stream: {:?} {:?}", stream, err);
                }
            }
        }
    }

    let mut consumer = environment
        .consumer()
        .offset(OffsetSpecification::First)
        .build(stream)
        .await
        .unwrap();

    let handle = consumer.handle();
    task::spawn(async move {
        while let Some(delivery) = consumer.next().await {
            let d = delivery.unwrap();
            println!(
                "Got message: {:#?} with offset: {}",
                d.message()
                    .data()
                    .map(|data| String::from_utf8(data.to_vec()).unwrap()),
                d.offset(),
            );
        }
    });
    println!("press any key to exit");

    _ = stdin().read_line(&mut String::new());
    handle.close().await.unwrap();
    print!("Closed consumer");
    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(receive()).unwrap();
    let _ = stdin().read_line(&mut String::new());
}
