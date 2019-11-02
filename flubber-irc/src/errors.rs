use irc::error::IrcError;

#[derive(Debug)]
pub enum Error {
	Irc(IrcError),
}

impl From<IrcError> for Error {
	fn from(err: IrcError) -> Self {
		Error::Irc(err)
	}
}