use hello::hello::{
    hello_service_server::HelloService, hello_service_server::HelloServiceServer, HelloRequest,
    HelloResponse,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

mod hello;

#[derive(Debug, Default)]
struct MyHelloServer;

#[tonic::async_trait]
impl HelloService for MyHelloServer {
    async fn hello_world(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let params = request.into_inner();
        let message = format!("Hello World! {}", params.name);
        Ok(Response::new(HelloResponse { message }))
    }

    type hello_world_server_streamStream = ReceiverStream<Result<HelloResponse, Status>>;

    async fn hello_world_server_stream(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<Self::hello_world_server_streamStream>, Status> {
        let params = request.into_inner();
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            for _ in 0..5 {
                let _ = tx
                    .send(Ok(HelloResponse {
                        message: format!("Hello World, {}!", params.name),
                    }))
                    .await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type hello_world_bidirecional_streamStream = ReceiverStream<Result<HelloResponse, Status>>;

    async fn hello_world_bidirecional_stream(
        &self,
        request: Request<tonic::Streaming<HelloRequest>>,
    ) -> Result<Response<Self::hello_world_bidirecional_streamStream>, Status> {
        let mut in_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(req) = in_stream.message().await.unwrap() {
                let _ = tx.send(Ok(HelloResponse {
                    message: format!("Hello World, {}!", req.name),
                }))
                .await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let hello_server = MyHelloServer::default();

    println!("MyHelloServer listening on {}", addr);

    Server::builder()
        .add_service(HelloServiceServer::new(hello_server))
        .serve(addr)
        .await?;

    Ok(())
}
