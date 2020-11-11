use actix::{Actor, Context, Handler};
use actix::Message;
use std::thread;
use actix::clock::Duration;
use actix_rt::System;

struct ActorA;

impl Actor for ActorA {
    type Context = Context<Self>;
}

impl ActorA {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Message)]
#[rtype(result = "()")]
struct Fuck {
    msg: String
}

impl Handler<Fuck> for ActorA {
    type Result = ();

    fn handle(&mut self, msg: Fuck, _: &mut Context<Self>) {
        println!("我是真尼玛的服：{}", msg.msg);
    }
}

fn main() {
    let system = System::new("events");

    let at = ActorA.start();
    at.do_send(Fuck {
        msg: "沃妮马".to_owned(),
    });
    // let join_handle = thread::spawn(move || {
    //
    // });
    // join_handle.join();

    System::current().stop();
    system.run();
}