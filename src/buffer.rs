use std::cmp;
use std::collections::HashMap;

use uuid::Uuid;

type BufferList = HashMap<Uuid, Buffer>;

#[derive(Default)]
pub struct Message {}

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
    pub fn root(&self, buflist: &BufferList) -> Uuid {
        match self.parent {
            Some(parent) => buflist.get(&parent).unwrap().root(buflist),
            None => self.id,
        }
    }

    // pub fn max_width(&self, indent: u32, buflist: &BufferList) -> u32 {
    //     self.children.iter().fold(0, |acc, buf| {
    //         let max_width = buflist.get(&buf).unwrap().max_width(indent, buflist);
    //         cmp::max(acc, indent + max_width)
    //     })
    // }
}
