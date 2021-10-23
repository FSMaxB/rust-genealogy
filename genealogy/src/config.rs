use crate::helpers::completable_future::CompletableFuture;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::files::Files;
use crate::helpers::list::List;
use crate::helpers::optional::Optional;
use crate::helpers::path::Path;
use crate::helpers::string::JString;
use crate::helpers::system::System;
use crate::{r#static, throw};

/// ```java
/// public record Config(
/// 	Path articleFolder,
/// 	Path talkFolder,
/// 	Path videoFolder,
/// 	Optional<Path> outputFile) {
/// ```
pub struct Config {
	pub article_folder: Path,
	pub talk_folder: Path,
	pub video_folder: Path,
	pub output_file: Optional<Path>,
}

impl Config {
	// ```java
	// private static final String CONFIG_FILE_NAME = "recommendations.config";
	// ```
	r#static!(pub CONFIG_FILE_NAME: JString = "recommendations.config".into());

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
			output_file: Optional::empty(),
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
	fn from_raw_config(raw: List<JString>) -> Result<Config, Exception> {
		#[allow(clippy::len_zero)]
		if raw.len() == 0 {
			throw!(IllegalArgumentException("No article path defined".into()));
		}

		let article_folder = Self::read_folder(raw.get(0)?)?;
		let talk_folder = Self::read_folder(raw.get(1)?)?;
		let video_folder = Self::read_folder(raw.get(2)?)?;

		let output_filename = if raw.len() >= 4 {
			Optional::of(raw.get(3)?)
		} else {
			Optional::empty()
		};

		let output_file = output_filename
			.map(|file| Ok::<_, Exception>(Path::of(System::get_property("user.dir")?).resolve(file)))?;
		output_file.if_present(|file| {
			let not_writable = Files::exists(file.clone()) && Files::is_writable(file.clone());
			if not_writable {
				throw!(IllegalArgumentException("Output path is not writable: " + file));
			}
			Ok(())
		})?;

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
	fn read_folder(raw: JString) -> Result<Path, Exception> {
		// NOTE: In general, paths are NOT valid Unicode strings.
		// E.g. on UNIX they are just bytes with some disallowed characters.
		let folder = Path::of(raw);

		if !Files::exists(&folder) {
			return Err(IllegalArgumentException("Path doesn't exist: " + &folder));
		}

		if !Files::is_directory(&folder) {
			return Err(IllegalArgumentException("Path is no directory: " + &folder));
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
	pub fn create(args: List<JString>) -> Result<CompletableFuture<Config>, Exception> {
		#[allow(clippy::len_zero)]
		let raw_config = if args.len() > 0 {
			CompletableFuture::completed_future(args)
		} else {
			Self::read_project_config()?
		}
		.exceptionally_compose_async(|_| Self::read_user_config())
		.exceptionally_compose(|_| Ok(List::new()));

		Ok(raw_config.then_apply(Config::from_raw_config))
	}

	/// ```java
	/// private static CompletableFuture<String[]> readProjectConfig() {
	/// 	var workingDir = Path.of(System.getProperty("user.dir")).resolve(CONFIG_FILE_NAME);
	/// 	return readConfig(workingDir);
	/// }
	/// ```
	fn read_project_config() -> Result<CompletableFuture<List<JString>>, Exception> {
		let working_dir = Path::of(System::get_property("user.dir")?).resolve(Self::CONFIG_FILE_NAME());
		Ok(Self::read_config(working_dir))
	}

	/// ```java
	/// private static CompletableFuture<String[]> readUserConfig() {
	/// 	var workingDir = Path.of(System.getProperty("user.home")).resolve(CONFIG_FILE_NAME);
	/// 	return readConfig(workingDir);
	/// }
	/// ```
	fn read_user_config() -> Result<CompletableFuture<List<JString>>, Exception> {
		let working_dir = Path::of(System::get_property("user.home")?).resolve(Self::CONFIG_FILE_NAME());
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
	fn read_config(working_dir: Path) -> CompletableFuture<List<JString>> {
		CompletableFuture::supply_async(move || Files::read_all_lines(&working_dir))
	}
}
