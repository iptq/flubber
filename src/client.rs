use futures::{future, Future};

use crate::Error;

pub struct Client {}

impl Client {
    pub fn new() -> Client {
        Client {}
    }

    pub fn run(self) -> impl Future<Item = (), Error = Error> {
        future::ok(())
    }
}
