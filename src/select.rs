use std::collections::{HashSet, hash_set::Iter as HashSetIter};

use futures::{Stream, Poll, Async};

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
