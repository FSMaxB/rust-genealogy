use crate::helpers::exception::Exception;
use crate::helpers::string::JString;
use url::Url;

#[derive(Debug)]
pub struct URI(Url);

impl URI {
	pub fn new(text: JString) -> Result<Self, Exception> {
		Ok(URI(Url::parse(text.as_ref())?))
	}
}
