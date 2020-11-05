use actix::prelude::*;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
struct Ping;

// Define actor
struct MyActor;

// Provide Actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}

#[actix_rt::main]
async fn main() {
    // Start MyActor in current thread
    let addr = MyActor.start();

    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let result = addr.send(Ping).await;

    match result {
        Ok(res) => println!("Got result: {}", res.unwrap()),
        Err(err) => println!("Got error: {}", err),
    }
}
