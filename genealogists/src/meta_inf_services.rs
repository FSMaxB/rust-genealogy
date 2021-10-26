use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy_java_apis::service_loader::ServiceLoader;

/// ```java
/// # org.codefx.java_after_eight.genealogists.repo.RepoGenealogist
/// # org.codefx.java_after_eight.genealogists.silly.SillyGenealogistService
/// org.codefx.java_after_eight.genealogists.tags.TagGenealogistService
/// # org.codefx.java_after_eight.genealogists.type.TypeGenealogist
/// ```
/// NOTE: This needs to be manually called on program start because there
/// is no way to run one time global initialization in rust without an
/// explicit call somewhere.
pub fn meta_inf_services() {
	ServiceLoader::register_many([
		//GenealogistService::from(crate::repo::repo_genealogist_service::RepoGenealogistService),
		//GenealogistService::from(crate::silly::silly_genealogist_service::SillyGenealogistService),
		GenealogistService::from(crate::tags::tag_genealogist_service::TagGenealogistService),
		//GenealogistService::from(crate::r#type::type_genealogist_service::TypeGenealogistService),
	])
}
