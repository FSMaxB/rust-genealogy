// NOTE: 😜
pub trait GetClass {
	fn get_class(&self) -> Class;
}

pub struct Class {
	simple_name: &'static str,
}

impl Class {
	pub fn get_simple_name(&self) -> &'static str {
		self.simple_name
	}
}

impl From<&'static str> for Class {
	fn from(simple_name: &'static str) -> Self {
		Self { simple_name }
	}
}
