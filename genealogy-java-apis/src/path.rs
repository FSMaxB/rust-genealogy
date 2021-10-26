use crate::string::JString;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Path {
	path: Rc<PathBuf>,
}

impl Path {
	pub fn of(path: impl AsRef<std::path::Path>) -> Path {
		path.as_ref().into()
	}

	pub fn resolve(&self, other: impl AsRef<std::path::Path>) -> Path {
		self.path.join(other.as_ref()).into()
	}
}

impl From<&std::path::Path> for Path {
	fn from(path: &std::path::Path) -> Self {
		Self {
			path: Rc::new(path.into()),
		}
	}
}

impl From<PathBuf> for Path {
	fn from(path: PathBuf) -> Self {
		Self { path: Rc::new(path) }
	}
}

impl AsRef<std::path::Path> for Path {
	fn as_ref(&self) -> &std::path::Path {
		self.path.as_ref()
	}
}

impl Add<Path> for &str {
	type Output = JString;

	fn add(self, right_hand_side: Path) -> Self::Output {
		format!("{}{}", self, right_hand_side).into()
	}
}

impl Add<&Path> for &str {
	type Output = JString;

	fn add(self, right_hand_side: &Path) -> Self::Output {
		format!("{}{}", self, right_hand_side).into()
	}
}

impl Display for Path {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		self.as_ref().display().fmt(formatter)
	}
}
