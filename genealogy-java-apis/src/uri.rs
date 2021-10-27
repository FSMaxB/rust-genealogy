use crate::exception::Exception;
use crate::string::JString;
use std::fmt::{Display, Formatter};
use url::Url;

#[derive(Clone, Debug)]
pub struct URI(Url);

impl URI {
	pub fn new(text: JString) -> Result<Self, Exception> {
		Ok(URI(Url::parse(text.as_ref())?))
	}
}

impl Display for URI {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		self.0.fmt(formatter)
	}
}
