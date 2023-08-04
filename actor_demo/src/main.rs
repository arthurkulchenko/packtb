use actix::prelude::*;
use actix::{Actor, Context, System};

use std::time::Duration;
// use std::time::Instant;
// use futures::future::Future;
// use futures::future;

struct Add(u32, u32);

// Result type of a message handler. Simply output of an Actor
impl Message for Add {
    type Result = Result<u32, ()>;
}

// ACTOR send and recieve messages of type Add.
struct Adder;

impl Actor for Adder {
    // type Context = SyncContext<Self>;
    type Context = Context<Self>;
}

impl Handler<Add> for Adder {
    type Result = Result<u32, ()>;

    fn handle(&mut self, msg: Add, _: &mut Self::Context) -> Self::Result {
        let sum = msg.0 + msg.1;
        println!("Computed: {} + {} = {}", msg.0, msg.1, sum);
        Ok(sum)
    }
}


#[actix::main]
async fn main() {
    let addr = Adder.start();
    let res = addr.send(Add(10, 5)).await; // <- send message and get future for result

    match res {
        Ok(result) => println!("SUM: {:?}", result),
        _ => println!("Communication to the actor has failed"),
    }

    // let mut system = System::new();
    // let addr = system.block_on(async {
    //     Game::create(|ctx| {
    //         // now we can get an address of the first actor and create the second actor
    //         let addr = ctx.address();

    //         let addr2 = Game {
    //             counter: 0,
    //             name: String::from("Game 2"),
    //             recipient: addr.recipient(),
    //         }
    //         .start();

    //         // let's start pings
    //         addr2.do_send(Ping { id: 10 });

    //         // now we can finally create first actor
    //         Game {
    //             counter: 0,
    //             name: String::from("Game 1"),
    //             recipient: addr2.recipient(),
    //         }
    //     });
    // });

    // system.run();
}


#[derive(Message)]
#[rtype(result = "()")]
struct Ping {
    pub id: usize,
}

// Actor definition
struct Game {
    counter: usize,
    name: String,
    recipient: Recipient<Ping>,
}

impl Actor for Game {
    type Context = Context<Game>;
}

// simple message handler for Ping message
impl Handler<Ping> for Game {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;

        if self.counter > 10 {
            System::current().stop();
        } else {
            println!("[{0}] Ping received {1}", self.name, msg.id);

            // wait 100 nanoseconds
            ctx.run_later(Duration::new(0, 100), move |act, _| {
                act.recipient.do_send(Ping { id: msg.id + 1 });
            });
        }
    }
}
