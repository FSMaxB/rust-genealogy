pub enum Objects {}

impl Objects {
	pub fn equals<A, B>(a: A, b: B) -> bool
	where
		A: PartialEq<B>,
	{
		a == b
	}
}
