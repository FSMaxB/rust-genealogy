use crate::config::Config;
use crate::helpers::exception::Exception;

mod config;
pub mod genealogist;
mod genealogy;
pub(crate) mod helpers;
pub mod post;
mod process_details;
mod recommendation;
#[cfg(test)]
pub mod test_helpers;
#[cfg(test)]
pub mod text_parser_tests;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Exception> {
	println!("{}", process_details::process_details());

	// NOTE: The first parameter is just the current program, so needs to be skipped.
	let args = std::env::args().skip(1).collect();
	let config = Config::create(args).await?;
	dbg!(config);
	// TODO: Implement this further
	Ok(())
}
