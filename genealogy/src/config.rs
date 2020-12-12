use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use directories::UserDirs;
use resiter::Map;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Config {
	pub article_folder: PathBuf,
	pub talk_folder: PathBuf,
	pub video_folder: PathBuf,
	pub output_file: Option<PathBuf>,
}

impl Config {
	pub fn create(args: Vec<String>) -> Result<Config, Exception> {
		let raw_config = if !args.is_empty() {
			args
		} else {
			match read_config(&project_config_directory()) {
				Ok(args) => args,
				Err(_) => read_config(&user_config_directory())?,
			}
		};
		from_raw_config(raw_config)
	}
}

fn project_config_directory() -> PathBuf {
	let mut working_directory = std::env::current_dir().expect("Failed to get working directory.");
	working_directory.push(CONFIG_FILE_NAME);
	working_directory
}

fn user_config_directory() -> PathBuf {
	let user_dirs = UserDirs::new().expect("Failed to find home directory.");
	// WTF: Why would you store config files in the home directory, this is just rude! There's proper directories for that.
	let mut home_directory = user_dirs.home_dir().to_path_buf();
	home_directory.push(CONFIG_FILE_NAME);
	home_directory
}

fn read_config(path: &Path) -> Result<Vec<String>, Exception> {
	let config_file = File::open(path)?;
	BufReader::new(config_file).lines().map_err(Exception::from).collect()
}

fn from_raw_config(raw: Vec<String>) -> Result<Config, Exception> {
	if raw.is_empty() {
		return Err(IllegalArgument("No article path defined.".into()));
	}

	let article_folder = read_folder(&raw[0])?;
	// WTF: Why would you only check for length >0 and then proceed to index potentially out of bounds?
	let talk_folder = read_folder(&raw[1])?;
	let video_folder = read_folder(&raw[2])?;

	let output_filename = if raw.len() >= 4 { Some(&raw[3]) } else { None };

	let output_file = output_filename.map(|file| {
		// NOTE: My attempt at replicating `System.getProperty("user.dir")`
		let mut working_directory = std::env::current_dir().expect("Failed to get working directory.");
		working_directory.push(file);
		working_directory
	});
	if let Some(output_file) = &output_file {
		// NOTE: The availability of metadata is used as the indicator if the file exists.
		let not_writable = output_file
			.metadata()
			.map(|metadata| metadata.permissions().readonly())
			.unwrap_or(true);
		if not_writable {
			return Err(IllegalArgument(format!(
				"Output path is not writable: {}",
				output_file.to_string_lossy()
			)));
		}
	}

	Ok(Config {
		article_folder,
		talk_folder,
		video_folder,
		output_file,
	})
}

fn read_folder(raw: &str) -> Result<PathBuf, Exception> {
	let folder = PathBuf::from(raw);

	// NOTE: In general, paths are NOT valid Unicode strings.
	// E.g. on UNIX they are just bytes with some disallowed characters.
	if !folder.exists() {
		return Err(IllegalArgument(format!(
			"Path doesn't exist: {}",
			folder.to_string_lossy()
		)));
	}

	if !folder.is_dir() {
		return Err(IllegalArgument(format!(
			"Path is no directory: {}",
			folder.to_string_lossy()
		)));
	}

	Ok(folder)
}

const CONFIG_FILE_NAME: &str = ".recs.config";
