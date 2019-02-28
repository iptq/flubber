// Copyright 2019 Nathan Ringo

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use std::collections::HashMap;
use std::hash::Hash;

use futures::{Async, Poll, Stream};

use crate::errors::Error;

pub struct SelectSet<K: Clone + Eq + Hash, S: Stream> {
    current: usize,
    keys: Vec<K>,
    streams: HashMap<K, S>,
}

impl<K: Clone + Eq + Hash, S: Stream> SelectSet<K, S> {
    pub fn new() -> SelectSet<K, S> {
        SelectSet::default()
    }

    pub fn add(&mut self, key: K, stream: S) -> Option<S> {
        if let Some(prev) = self.streams.insert(key.clone(), stream) {
            Some(prev)
        } else {
            self.keys.push(key);
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<S> {
        self.streams.remove(key).map(|stream| {
            let n = self.keys.iter().position(|k| k == key).unwrap();
            self.keys.remove(n);

            stream
        })
    }
}

impl<K: Clone + Eq + Hash, S: Stream> Default for SelectSet<K, S> {
    fn default() -> SelectSet<K, S> {
        SelectSet {
            current: 0,
            keys: Vec::new(),
            streams: HashMap::new(),
        }
    }
}

impl<K: Clone + Eq + Hash, S: Stream> Stream for SelectSet<K, S> {
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Result<Async<Option<S::Item>>, S::Error> {
        if self.keys.is_empty() {
            return Ok(Async::NotReady);
        }

        self.current = (self.current + 1) % self.keys.len();
        let r = self
            .streams
            .get_mut(&self.keys[self.current])
            .unwrap()
            .poll();

        if let Ok(Async::Ready(None)) = r {
            let key = self.keys[self.current].clone();
            self.remove(&key);
        }
        r
    }
}
