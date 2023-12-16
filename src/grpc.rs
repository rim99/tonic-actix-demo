use crate::hello_world::{HelloReply, HelloRequest};
use crate::hello_world::greeter_server::{GreeterServer, Greeter};
use crate::actor::{Ping, MyActor};
use tonic::{Request, Response, Status};
use actix::Addr;

pub fn create(actor_addr: Addr<MyActor>) -> GreeterServer<MyGreeter>  {
    let greeter = MyGreeter { actor_addr };
    GreeterServer::new(greeter)
}

pub struct MyGreeter {
    actor_addr: Addr<MyActor>,
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let cnt = self.actor_addr.send(Ping(10)).await;

        let reply = HelloReply {
            message: format!(
                "Hello {}! Count: {}",
                request.into_inner().name,
                cnt.unwrap()
            ),
        };
        Ok(Response::new(reply))
    }
}