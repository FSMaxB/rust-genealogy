use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::GenealogistTrait;
use genealogy::helpers::collector::Collectors;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::integer::Integer;
use genealogy::helpers::set::{JHashSet, Set};
use genealogy::post::Post;
use genealogy::r#static;

/// ```java
/// public class SillyGenealogist implements Genealogist {
/// ```
pub struct SillyGenealogist;

/// ```java
/// public class SillyGenealogist implements Genealogist {
/// ```
impl GenealogistTrait for SillyGenealogist {
	/// ```java
	/// @Override
	///	public TypedRelation infer(Post post1, Post post2) {
	///		var post1Letters = titleLetters(post1);
	///		var post2Letters = titleLetters(post2);
	///		var intersection = new HashSet<>(post1Letters);
	///		intersection.retainAll(post2Letters);
	///		long score = round((100.0 * intersection.size()) / post1Letters.size());
	///
	///		return new TypedRelation(post1, post2, TYPE, score);
	///	}
	/// ```
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		let post1_letters = Self::title_letters(post1.clone())?;
		let post2_letters = Self::title_letters(post2.clone())?;
		let intersection = JHashSet::new();
		intersection.retain_all(post2_letters);
		let score = ((100.0 * (intersection.size() as f64)) / (post1_letters.size() as f64)).round() as i64;

		TypedRelation::new(post1, post2, Self::TYPE(), score)
	}
}

/// ```java
/// public class SillyGenealogist implements Genealogist {
/// ```
impl SillyGenealogist {
	r#static!(TYPE: RelationType = RelationType::new("silly".into()).unwrap());

	/// ```java
	/// public class SillyGenealogist implements Genealogist {
	/// ```
	pub fn new() -> Self {
		Self
	}

	/// ```java
	/// private static Set<Integer> titleLetters(Post post) {
	///		return post
	///				.title()
	///				.text()
	///				.toLowerCase()
	///				.chars().boxed()
	///				.collect(toUnmodifiableSet());
	///	}
	/// ```
	fn title_letters(post: Post) -> Result<Set<Integer>, Exception> {
		post.title()
			.text
			.to_lower_case()
			.chars()
			.boxed()
			.collect(Collectors::to_unmodifiable_set())
	}
}
