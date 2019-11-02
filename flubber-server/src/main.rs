use std::path::Path;

#[derive(Debug)]
enum Error {
	Fuck,
}

fn spawn_plugin(path: impl AsRef<Path>) {
	
}

fn main() -> Result<(), Error> {
	Err(Error::Fuck)
}
