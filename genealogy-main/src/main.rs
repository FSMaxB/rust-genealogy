use genealogists::r#type::type_genealogist_service::TypeGenealogistService;
use genealogists::repo::repo_genealogist_service::RepoGenealogistService;
use genealogists::silly::silly_genealogist_service::SillyGenealogistService;
use genealogists::tags::tag_genealogist_service::TagGenealogistService;
use genealogy::config::Config;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::genealogy::weights::Weights;
use genealogy::genealogy::Genealogy;
use genealogy::helpers::exception::Exception;
use genealogy::post::factories::article_factory::ArticleFactory;
use genealogy::post::factories::talk_factory::TalkFactory;
use genealogy::post::factories::video_factory::VideoFactory;
use genealogy::post::Post;
use genealogy::process_details;
use genealogy::recommendation::recommender::Recommender;
use genealogy::recommendation::Recommendation;
use genealogy::utils::{unchecked_files_list, unchecked_files_write};
use resiter::{AndThen, Filter, Map};
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Exception> {
	println!("{}", process_details::process_details());

	// NOTE: The first parameter is just the current program, so needs to be skipped.
	let args = std::env::args().skip(1).collect();
	let config = Config::create(args).await?;
	let genealogy = create_genealogy(&config.article_folder, &config.talk_folder, &config.video_folder)?;

	let relations = genealogy.infer_relations();
	let recommendations = Recommender::recommend(relations, 3)?;
	let recommendations_as_json = recommendations_to_json(recommendations);
	if let Some(output_file) = &config.output_file {
		unchecked_files_write(output_file, &recommendations_as_json)?;
	}
	Ok(())
}

fn create_genealogy(
	article_folder: &PathBuf,
	talk_folder: &PathBuf,
	video_folder: &PathBuf,
) -> Result<Genealogy, Exception> {
	let posts: Vec<Box<dyn Iterator<Item = Result<Post, Exception>>>> = vec![
		Box::new(
			markdown_files_in(article_folder)
				.and_then_ok(|path| ArticleFactory::create_article_from_path(&path).map(Post::Article)),
		),
		Box::new(markdown_files_in(talk_folder).and_then_ok(|path| TalkFactory::create_talk(&path).map(Post::Talk))),
		Box::new(
			markdown_files_in(video_folder).and_then_ok(|path| VideoFactory::create_video(&path).map(Post::Video)),
		),
	];
	let posts = posts
		.into_iter()
		.flatten()
		.map_ok(Arc::new)
		.collect::<Result<Vec<_>, _>>()?;
	let genealogists = get_genealogists(posts.clone());
	Ok(Genealogy::new(posts, genealogists, Arc::new(Weights::all_equal())))
}

fn markdown_files_in(folder: &PathBuf) -> impl Iterator<Item = Result<PathBuf, Exception>> {
	unchecked_files_list(folder)
		.filter_ok(|path| path.is_file())
		.filter_ok(|path| path.ends_with(".md"))
}

fn get_genealogists(posts: Vec<Arc<Post>>) -> Vec<Arc<dyn Genealogist>> {
	// NOTE: Not quite dynamic class loading, but hey, that's just not possible in Rust
	let genealogist_services: Vec<Box<dyn GenealogistService>> = vec![
		Box::new(SillyGenealogistService),
		Box::new(TagGenealogistService),
		Box::new(RepoGenealogistService),
		Box::new(TypeGenealogistService),
	];
	genealogist_services
		.into_iter()
		.map(move |service| service.procure(Box::new(posts.clone().into_iter())))
		.collect()
}

// WTF: W.T.F. Don't build your own JSON, you just don't ever do that!
fn recommendations_to_json(recommendations: impl Iterator<Item = Recommendation>) -> String {
	const FRAME: &str = r#"[
$RECOMMENDATIONS
]
"#;
	const RECOMMENDATION: &str = r#"	{
		"title": "$TITLE",
		"recommendations": [
	$RECOMMENDED_POSTS
		]
	}
	"#;

	const RECOMMENDED_POSTS: &str = r#"			{ "title": "$TITLE" }"#;

	let recs = recommendations
		.map(|rec| {
			let posts = rec
				.recommended_posts()
				.iter()
				.map(|rec_art| rec_art.title())
				.map(|rec_title| RECOMMENDED_POSTS.replace("$TITLE", &rec_title.text))
				.collect::<Vec<_>>()
				.join(",\n");
			RECOMMENDATION
				.replace("$TITLE", &rec.post().title().text)
				.replace("$RECOMMENDED_POSTS", &posts)
		})
		.collect::<Vec<_>>()
		.join(",\n");
	FRAME.replace("$RECOMMENDATIONS", &recs)
}
