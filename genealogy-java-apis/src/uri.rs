use crate::exception::Exception;
use crate::string::JString;
use url::Url;

#[derive(Clone, Debug)]
pub struct URI(Url);

impl URI {
	pub fn new(text: JString) -> Result<Self, Exception> {
		Ok(URI(Url::parse(text.as_ref())?))
	}
}
