pub mod hello_world {
    tonic::include_proto!("helloworld");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("helloworld_descriptor");
}
mod actor;
mod grpc;

use actix_rt::System;
use tonic::transport::Server;


fn main() {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let system = System::new();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(hello_world::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let actor_addr = actor::create();
    let grpc_server = Server::builder()
        .add_service(reflection_service)
        .add_service(grpc::create(actor_addr))
        .serve(addr);

    system.runtime().spawn(grpc_server);

    println!("GrpcServer listening on {}", addr);
    let _ = system.run();
}
