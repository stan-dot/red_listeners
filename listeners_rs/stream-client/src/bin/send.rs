use rabbitmq_stream_client::error::StreamCreateError;
use rabbitmq_stream_client::types::{ByteCapacity, Message, ResponseCode};

use rabbitmq_stream_client::{Environment, Producer};

async fn send() -> Result<(), StreamCreateError> {
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
    let producer_result = environment.producer().build(stream).await;
    let producer = match producer_result {
        Ok(producer) => producer,
        Err(e) => {
            println!("Error creating producer: {:?}", e);
            return Ok(());
        }
    };

    let response = producer
        .send_with_confirm(Message::builder().body("Hello, World!").build())
        .await
        .unwrap();

    println!("Message sent: {:?}", response);

    Ok(())
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(send()).unwrap();
}
