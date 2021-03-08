use tonic::{transport::Server, Request, Response, Status};
use hades::connection_api_server::{ConnectionApi, ConnectionApiServer};
use hades::room_api_server::{RoomApi, RoomApiServer};
use hades::*;
use std::net::SocketAddr;
use std::error::Error;
use std::future::Future;

pub mod hades {
    tonic::include_proto!("hades");
}

#[derive(Debug, Default)]
pub struct ConnectionGRPC {}

#[tonic::async_trait]
impl ConnectionApi for ConnectionGRPC {
    async fn create_connection(&self, request: tonic::Request<ConnectionRequest>) -> Result<tonic::Response<CreateConnectionReply>, tonic::Status> {
        unimplemented!()
    }

    async fn delete_connection(&self, request: tonic::Request<ConnectionId>) -> Result<tonic::Response<Successful>, tonic::Status> {
        unimplemented!()
    }

    async fn update_connection(&self, request: tonic::Request<ConnectionRequest>) -> Result<tonic::Response<Successful>, tonic::Status> {
        unimplemented!()
    }

    async fn get_connection(&self, request: tonic::Request<ConnectionId>) -> Result<tonic::Response<Connection>, tonic::Status> {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct RoomGRPC {}

#[tonic::async_trait]
impl RoomApi for RoomGRPC {
    async fn create_room(&self, request: tonic::Request<Room>) -> Result<tonic::Response<Successful>, tonic::Status> {
        unimplemented!()
    }

    async fn delete_room(&self, request: tonic::Request<RoomId>) -> Result<tonic::Response<Successful>, tonic::Status> {
        unimplemented!()
    }

    async fn update_room(&self, request: tonic::Request<Room>) -> Result<tonic::Response<Successful>, tonic::Status> {
        unimplemented!()
    }

    async fn get_room(&self, request: tonic::Request<RoomId>) -> Result<tonic::Response<Room>, tonic::Status> {
        unimplemented!()
    }
}

pub async fn serve(addr: SocketAddr) -> Result<(), tonic::transport::Error> {
    let connection_api_server = ConnectionGRPC::default();
    let room_api_server = RoomGRPC::default();

    Server::builder()
        .add_service(ConnectionApiServer::new(connection_api_server))
        .add_service(RoomApiServer::new(room_api_server))
        .serve(addr).await
}