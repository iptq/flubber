use std::collections::{hash_set::Iter as HashSetIter, HashSet};

use futures::{Async, Poll, Stream};

use crate::errors::Error;

pub struct SelectSet<T>(HashSet<T>);

impl<T> SelectSet<T> {
    pub fn add() {}
}

impl<T> Stream for SelectSet<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // TODO:
        Ok(Async::NotReady)
    }
}
