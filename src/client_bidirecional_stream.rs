use hello::hello::hello_service_client::HelloServiceClient;
use hello::hello::HelloRequest;
use tokio_stream::StreamExt;
use futures::stream::iter;

mod hello;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = HelloServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(iter(
        vec![
            HelloRequest {
                name: "João".into(),
            },
            HelloRequest {
                name: "Maria".into(),
            },
            HelloRequest {
                name: "Paulo".into(),
            },
            HelloRequest {
                name: "Pedro".into(),
            }
        ],
    ));

    let response = client.hello_world_bidirecional_stream(request).await?;
    let mut stream = response.into_inner();

    while let Some(item) = stream.next().await {
        let item = item?;
        println!("RESPONSE={:?}", item.message);
    }

    Ok(())
}
