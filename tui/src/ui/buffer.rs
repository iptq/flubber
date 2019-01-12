use uuid::Uuid;

pub struct Message {}

pub struct Buffer {
    pub id: Uuid,
    system: bool,
    topic: String,
    messages: Vec<Message>,
}
