use crate::helpers::completable_future::CompletableFuture;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::files::Files;
use crate::helpers::indexing::index;
use crate::helpers::path::Path;
use crate::helpers::system::System;
use crate::throw;
use std::path::PathBuf;

/// ```java
/// public record Config(
/// 	Path articleFolder,
/// 	Path talkFolder,
/// 	Path videoFolder,
/// 	Optional<Path> outputFile) {
/// ```
pub struct Config {
	pub article_folder: PathBuf,
	pub talk_folder: PathBuf,
	pub video_folder: PathBuf,
	pub output_file: Option<PathBuf>,
}

impl Config {
	/// ```java
	/// private static final String CONFIG_FILE_NAME = "recommendations.config";
	/// ```
	const CONFIG_FILE_NAME: &'static str = "recommendations.config";

	/// ```java
	/// // use static factory method(s)
	/// @Deprecated
	/// public Config { }
	/// ```
	#[deprecated(note = "use static factory methods")]
	pub fn constructor() -> Self {
		Self {
			article_folder: Default::default(),
			talk_folder: Default::default(),
			video_folder: Default::default(),
			output_file: None,
		}
	}

	/// ```java
	/// private static Config fromRawConfig(String[] raw) {
	/// 	if (raw.length == 0)
	/// 		throw new IllegalArgumentException("No article path defined.");
	///
	///		var articleFolder = readFolder(raw[0]);
	///		var talkFolder = readFolder(raw[1]);
	///		var videoFolder = readFolder(raw[2]);
	///
	///		Optional<String> outputFileName = raw.length >= 4
	///				? Optional.of(raw[3])
	///				: Optional.empty();
	///		var outputFile = outputFileName
	///				.map(file -> Path.of(System.getProperty("user.dir")).resolve(file));
	///		outputFile.ifPresent(file -> {
	///			boolean notWritable = Files.exists(file) && !Files.isWritable(file);
	///			if (notWritable)
	///				throw new IllegalArgumentException("Output path is not writable: " + outputFile.get());
	///		});
	///
	///		return new Config(articleFolder, talkFolder, videoFolder, outputFile);
	///	}
	/// ```
	fn from_raw_config(raw: Vec<String>) -> Result<Config, Exception> {
		#[allow(clippy::len_zero)]
		if raw.len() == 0 {
			throw!(IllegalArgumentException("No article path defined".into()));
		}

		let article_folder = Self::read_folder(index(&raw, 0)?)?;
		let talk_folder = Self::read_folder(index(&raw, 1)?)?;
		let video_folder = Self::read_folder(index(&raw, 2)?)?;

		let output_filename = if raw.len() >= 4 { Some(index(&raw, 3)?) } else { None };

		let output_file = output_filename
			.map(|file| Ok::<_, Exception>(Path::of(&System::get_property("user.dir")?).join(file)))
			.transpose()?;
		if let Some(output_file) = &output_file {
			let not_writable = output_file.exists() && Files::is_writable(output_file);
			if not_writable {
				throw!(IllegalArgumentException(format!(
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

	/// ```java
	/// private static Path readFolder(String raw) {
	/// 	var folder = Path.of(raw);
	/// 	if (!Files.exists(folder))
	/// 		throw new IllegalArgumentException("Path doesn't exist: " + folder);
	/// 	if (!Files.isDirectory(folder))
	/// 		throw new IllegalArgumentException("Path is no directory: " + folder);
	/// 	return folder;
	/// }
	/// ```
	fn read_folder(raw: &str) -> Result<PathBuf, Exception> {
		let folder = PathBuf::from(raw);

		// NOTE: In general, paths are NOT valid Unicode strings.
		// E.g. on UNIX they are just bytes with some disallowed characters.
		if !folder.exists() {
			return Err(IllegalArgumentException(format!(
				"Path doesn't exist: {}",
				folder.to_string_lossy()
			)));
		}

		if !folder.is_dir() {
			return Err(IllegalArgumentException(format!(
				"Path is no directory: {}",
				folder.to_string_lossy()
			)));
		}

		Ok(folder)
	}

	/// ```java
	/// public static CompletableFuture<Config> create(String[] args) {
	///		CompletableFuture<String[]> rawConfig = args.length > 0
	///				? CompletableFuture.completedFuture(args)
	///				: readProjectConfig()
	///				.exceptionallyComposeAsync(__ -> readUserConfig())
	///				.exceptionallyAsync(__ -> new String[0]);
	///
	///		return rawConfig
	///				.thenApply(Config::fromRawConfig);
	///	}
	/// ```
	pub fn create(args: Vec<String>) -> Result<CompletableFuture<Config>, Exception> {
		#[allow(clippy::len_zero)]
		let raw_config = if args.len() > 0 {
			CompletableFuture::completed_future(args)
		} else {
			Self::read_project_config()?
		}
		.exceptionally_compose_async(|_| Self::read_user_config())
		.exceptionally_compose(|_| Ok(Vec::new()));

		Ok(raw_config.then_apply(Config::from_raw_config))
	}

	/// ```java
	/// private static CompletableFuture<String[]> readProjectConfig() {
	/// 	var workingDir = Path.of(System.getProperty("user.dir")).resolve(CONFIG_FILE_NAME);
	/// 	return readConfig(workingDir);
	/// }
	/// ```
	fn read_project_config() -> Result<CompletableFuture<Vec<String>>, Exception> {
		let working_dir = Path::of(&System::get_property("user.dir")?).join(Self::CONFIG_FILE_NAME);
		Ok(Self::read_config(working_dir))
	}

	/// ```java
	/// private static CompletableFuture<String[]> readUserConfig() {
	/// 	var workingDir = Path.of(System.getProperty("user.home")).resolve(CONFIG_FILE_NAME);
	/// 	return readConfig(workingDir);
	/// }
	/// ```
	fn read_user_config() -> Result<CompletableFuture<Vec<String>>, Exception> {
		let working_dir = Path::of(&System::get_property("user.home")?).join(Self::CONFIG_FILE_NAME);
		Ok(Self::read_config(working_dir))
	}

	/// ```java
	/// private static CompletableFuture<String[]> readConfig(Path workingDir) {
	/// 	return CompletableFuture.supplyAsync(() -> {
	/// 		try {
	/// 			return Files.readAllLines(workingDir).toArray(String[]::new);
	/// 		} catch (IOException ex) {
	/// 			throw new UncheckedIOException(ex);
	/// 		}
	/// 	});
	/// }
	/// ```
	fn read_config(working_dir: PathBuf) -> CompletableFuture<Vec<String>> {
		CompletableFuture::supply_async(move || {
			Files::read_all_lines(&working_dir)?.collect::<Result<Vec<String>, Exception>>()
		})
	}
}
