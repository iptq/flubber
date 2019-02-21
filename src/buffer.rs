use std::cmp;
use std::collections::HashMap;

use chrono::{offset::Utc, DateTime};
use uuid::Uuid;

type BufferList = HashMap<Uuid, Buffer>;

pub struct Message {
    timestamp: DateTime<Utc>,
    author: String,
    contents: String,
}

pub struct Buffer {
    title: String,
    contents: Vec<Message>,
    names: Vec<String>,

    id: Uuid,
    parent: Option<Uuid>,
    children: Vec<Uuid>,
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            title: "flubber".to_owned(),
            contents: Vec::new(),
            names: Vec::new(),

            id: Uuid::new_v4(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl Buffer {
    pub fn root_buffer() -> Buffer {
        Buffer::default()
    }

    pub fn root(&self, buflist: &BufferList) -> Uuid {
        match self.parent {
            Some(parent) => buflist.get(&parent).unwrap().root(buflist),
            None => self.id,
        }
    }
}
