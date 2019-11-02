#[derive(Serialize, Deserialize)]
pub struct Version(pub u32, pub u32, pub u32);


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitInfo {
	pub plugin_name: String,
	pub plugin_version: Version,
	pub protocol_version: Version,
}

#[derive(Serialize, Deserialize)]
pub struct MessageID(pub String);

#[derive(Serialize, Deserialize)]
pub struct RoomID(pub String);

#[derive(Serialize, Deserialize)]
pub struct UserID(pub String);

#[derive(Serialize, Deserialize)]
pub enum Recipient {
	Room(RoomID),
	User(UserID),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
	pub id: MessageID,
	pub sender: UserID,
	pub recipient: Recipient,
	pub attachments: Vec<MessageAttachment>,
	pub content: MessageContent,
}

#[derive(Serialize, Deserialize)]
pub enum MessageContent {
    Bold(Box<MessageContent>),
    Concat(Vec<MessageContent>),
    Crossout(Box<MessageContent>),
    Emote(String),
    Italic(Box<MessageContent>),
    MessageLink(MessageID),
    RoomLink(RoomID),
    Text(String),
    UrlLink(String),
    Underline(Box<MessageContent>),
    UserLink(UserID),
}

#[derive(Serialize, Deserialize)]
pub struct MessageAttachment {
	mime: Option<String>,
	data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum Update {
	MessageUpsert(Message),
}