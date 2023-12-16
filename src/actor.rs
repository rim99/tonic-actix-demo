use actix::prelude::*;
use actix::Addr;

pub fn create() -> Addr<MyActor> {
    SyncArbiter::start(2, || MyActor { count: 10 })
}

pub struct MyActor {
    pub count: usize,
}

impl Actor for MyActor {
    type Context = SyncContext<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut SyncContext<Self>) -> Self::Result {
        self.count += msg.0;

        self.count
    }
}
