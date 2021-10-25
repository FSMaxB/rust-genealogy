use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Integer(Rc<i32>);

impl From<i32> for Integer {
	fn from(integer: i32) -> Self {
		Self(Rc::new(integer))
	}
}
