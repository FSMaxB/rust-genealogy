#![allow(clippy::tabs_in_doc_comments)]
use genealogists::meta_inf_services::meta_inf_services;
use genealogy::config::Config;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::genealogy::weights::Weights;
use genealogy::genealogy::Genealogy;
use genealogy::helpers::collection::Collection;
use genealogy::helpers::collector::Collectors;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::exception::Exception::IllegalArgumentException;
use genealogy::helpers::files::Files;
use genealogy::helpers::list::List;
use genealogy::helpers::path::Path;
use genealogy::helpers::service_loader::{Class, ServiceLoader};
use genealogy::helpers::stream::Stream;
use genealogy::helpers::string::JString;
use genealogy::helpers::system::System;
use genealogy::post::factories::article_factory::ArticleFactory;
use genealogy::post::factories::talk_factory::TalkFactory;
use genealogy::post::factories::video_factory::VideoFactory;
use genealogy::post::Post;
use genealogy::process_details::ProcessDetails;
use genealogy::recommendation::recommender::Recommender;
use genealogy::recommendation::Recommendation;
use genealogy::throw;
use genealogy::utils::Utils;
use std::env::args;

/// ```java
/// public class Main {
/// ```
pub struct Main;

impl Main {
	/// ```java
	/// public static void main(String[] args) {
	///		System.out.println(ProcessDetails.details());
	///
	///		var config = Config.create(args).join();
	///		var genealogy = createGenealogy(config.articleFolder(), config.talkFolder(), config.videoFolder());
	///		var recommender = new Recommender();
	///
	///		var relations = genealogy.inferRelations();
	///		var recommendations = recommender.recommend(relations, 3);
	///		var recommendationsAsJson = recommendationsToJson(recommendations);
	///
	///		config.outputFile().ifPresentOrElse(
	///				outputFile -> Utils.uncheckedFilesWrite(outputFile, recommendationsAsJson),
	///				() -> System.out.println(recommendationsAsJson));
	///	}
	/// ```
	pub fn main(args: List<JString>) -> Result<(), Exception> {
		System::out_println(ProcessDetails::details());

		let config = Config::create(args)?.join()?;
		let genealogy = Self::create_genealogy(config.article_folder, config.talk_folder, config.video_folder)?;
		let recommender = Recommender::new();

		let relations = genealogy.infer_relations()?;
		let recommendations = recommender.recommend(relations, 3)?;
		let recommendations_as_json = Self::recommendations_to_json(recommendations)?;

		config.output_file.if_present_or_else(
			|output_file| Utils::unchecked_files_write(output_file, recommendations_as_json.clone()),
			|| System::out_println(recommendations_as_json.clone()),
		)?;
		Ok(())
	}

	/// ```java
	/// private static Genealogy createGenealogy(Path articleFolder, Path talkFolder, Path videoFolder) {
	///		List<Post> posts = concat(
	///				markdownFilesIn(articleFolder).<Post>map(ArticleFactory::createArticle),
	///				markdownFilesIn(talkFolder).map(TalkFactory::createTalk),
	///				markdownFilesIn(videoFolder).map(VideoFactory::createVideo)
	///		).toList();
	///		Collection<Genealogist> genealogists = getGenealogists(posts);
	///		return new Genealogy(posts, genealogists, Weights.allEqual());
	///	}
	/// ```
	fn create_genealogy(article_folder: Path, talk_folder: Path, video_folder: Path) -> Result<Genealogy, Exception> {
		let posts = Utils::concat([
			Self::markdown_files_in(article_folder)?
				.map(ArticleFactory::create_article)
				.map(|article| Ok(Post::from(article))),
			Self::markdown_files_in(talk_folder)?
				.map(TalkFactory::create_talk)
				.map(|talk| Ok(Post::from(talk))),
			Self::markdown_files_in(video_folder)?
				.map(VideoFactory::create_video)
				.map(|video| Ok(Post::from(video))),
		])
		.to_list()?;
		let genealogists = Self::get_genealogists(posts.clone())?;
		Ok(Genealogy::new(posts, genealogists, Weights::all_equal()))
	}

	/// ```java
	/// private static Stream<Path> markdownFilesIn(Path folder) {
	///		return Utils.uncheckedFilesList(folder)
	///				.filter(Files::isRegularFile)
	///				.filter(file -> file.toString().endsWith(".md"));
	///	}
	/// ```
	fn markdown_files_in(folder: Path) -> Result<Stream<Path>, Exception> {
		#[allow(clippy::redundant_closure)] // doesn't compile without the closure
		Ok(Utils::unchecked_files_list(folder)?
			.filter(|file| Files::is_regular_file(file))
			.filter(|file| file.to_string().ends_with(".md")))
	}

	/// ```java
	/// private static Collection<Genealogist> getGenealogists(Collection<Post> posts) {
	///		var genealogists = ServiceLoader
	///				.load(GenealogistService.class).stream()
	///				.map(ServiceLoader.Provider::get)
	///				.map(service -> service.procure(posts))
	///				.toList();
	///		if (genealogists.isEmpty())
	///			throw new IllegalArgumentException("No genealogists found.");
	///		return genealogists;
	///	}
	/// ```
	fn get_genealogists(posts: Collection<Post>) -> Result<Collection<Genealogist>, Exception> {
		let genealogists = ServiceLoader::load(GenealogistService::class())
			.stream()?
			.map(|provider| Ok(provider.get()))
			.map(move |service| service.procure(posts.clone()))
			.to_list()?;
		if genealogists.is_empty() {
			throw!(IllegalArgumentException("No genealogists found.".into()));
		}
		Ok(genealogists)
	}

	/// ```java
	/// private static String recommendationsToJson(Stream<Recommendation> recommendations) {
	///		var frame = """
	///				[
	///				$RECOMMENDATIONS
	///				]
	///				""";
	///		var recommendation = """
	///					{
	///						"title": "$TITLE",
	///						"recommendations": [
	///				$RECOMMENDED_POSTS
	///						]
	///					}
	///				""";
	///		var recommendedPost = """
	///				\t\t\t{ "title": "$TITLE" }""";
	///
	///		var recs = recommendations
	///				.map(rec -> {
	///					String posts = rec
	///							.recommendedPosts().stream()
	///							.map(recArt -> recArt.title().text())
	///							.map(recTitle -> recommendedPost.replace("$TITLE", recTitle))
	///							.collect(joining(",\n"));
	///					return recommendation
	///							.replace("$TITLE", rec.post().title().text())
	///							.replace("$RECOMMENDED_POSTS", posts);
	///				})
	///				.collect(joining(",\n"));
	///		return frame.replace("$RECOMMENDATIONS", recs);
	///	}
	/// ```
	fn recommendations_to_json(recommendations: Stream<Recommendation>) -> Result<JString, Exception> {
		let frame = JString::from(
			r#"[
$RECOMMENDATIONS
]
"#,
		);
		let recommendation = JString::from(
			r#"	{
	"title": "$TITLE",
	"recommendations": [
$RECOMMENDED_POSTS
		]
	}
"#,
		);
		let recommended_post = JString::from(r#"			{ "title": "$TITLE" }"#);

		let recs = recommendations
			.map(move |rec| {
				let posts = rec
					.recommended_posts
					.stream()
					.map(|rec_art| Ok(rec_art.title().text.clone()))
					.map({
						let recommended_post = recommended_post.clone();
						move |rec_title| Ok(recommended_post.clone().replace("$TITLE", rec_title))
					})
					.collect(Collectors::joining(",\n"))?;
				Ok(recommendation
					.clone()
					.replace("$TITLE", &rec.post.title().text)
					.replace("$RECOMMENDED_POSTS", posts))
			})
			.collect(Collectors::joining(",\n"))?;
		Ok(frame.replace("$RECOMMENDATIONS", recs))
	}
}

fn main() -> Result<(), Exception> {
	// One time global initialization that makes sure the ServiceLoader knows all the services.
	// This can't be done automatically in Rust since there is no Reflection or class loader.
	meta_inf_services();

	// Note: Java doesn't get the first parameter as native processes do.
	Main::main(List::of(args().skip(1).map(JString::from)))
}
