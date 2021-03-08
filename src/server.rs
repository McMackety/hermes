mod grpc;

use grpc::hades::room_api_server::{RoomApiServer};
use grpc::hades::connection_api_server::{ConnectionApiServer};
use grpc::{RoomGRPC, ConnectionGRPC};
use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    grpc::serve(addr).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_say_hello() {
        let greeter = MyGreeter::default();
        let res = greeter.say_hello(tonic::Request::new(HelloRequest {
            name: String::from("Chase"),
        })).await.ok().expect("Response Not found");
        let rep = res.get_ref();
        assert_eq!(rep.message, "Hello Chase!");
    }
}