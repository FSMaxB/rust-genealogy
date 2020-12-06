use crate::config::Config;
use crate::java_replicas::exception::Exception;

mod config;
mod java_replicas;
mod post;
mod process_details;
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
