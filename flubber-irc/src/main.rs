mod errors;

use std::sync::mpsc;
use std::io::{self, Write};

use flubber::proto::{Message, *};
use irc::client::prelude::*;

use crate::errors::Error;

fn send_init_info() {
	// send a init info to stdout
	let init_info = InitInfo {
		plugin_name: "irc".to_owned(),
		plugin_version: Version(0, 1, 0),
		protocol_version: Version(0, 1, 0),
	};
	println!("{}", serde_json::to_string(&init_info).unwrap());
}

fn main() -> Result<(), Error> {
	send_init_info();

	let config1 = Config {
		nickname: Some("flubber".to_owned()),
		realname: Some("flubber".to_owned()),
		server: Some("acm.umn.edu".to_owned()),
		use_ssl: Some(true),
		port: Some(6669),
		channels: Some(vec!["#flubber".to_owned()]),
		..Config::default()
	};

	let config2 = Config {
		nickname: Some("flubber2".to_owned()),
		realname: Some("flubber2".to_owned()),
		server: Some("acm.umn.edu".to_owned()),
		use_ssl: Some(true),
		port: Some(6669),
		channels: Some(vec!["#flubber".to_owned()]),
		..Config::default()
	};

	let configs = vec![config1, config2];

	let mut reactor = IrcReactor::new().unwrap();
	for config in configs.iter() {
		let mut stdout = io::stdout();
		let client = reactor.prepare_client_and_connect(config).unwrap();
		client.identify().unwrap();
		reactor.register_client_with_handler(client, move |client, msg| {
			if let Command::PRIVMSG(target, content) = &msg.command{
				let recipient = if target.starts_with("#") {
					Recipient::Room(RoomID(target.to_owned()))
				} else {
					Recipient::User(UserID(target.to_owned()))
				};
				let message = Message {
					id: MessageID("".to_owned()),
					recipient,
					sender: UserID("".to_owned()),
					attachments: vec![],
					content: MessageContent::Text(content.to_owned()),
				};
				let update = Update::MessageUpsert(message);
				stdout.write_all(serde_json::to_string(&update).unwrap().as_bytes());
				stdout.flush();
			} else {
				// eprintln!("message: {:?}", msg);
			}
			Ok(())
		});
	}

	reactor.run().unwrap();
	unreachable!()
}