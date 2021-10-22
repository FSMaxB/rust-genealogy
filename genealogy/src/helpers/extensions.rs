pub trait Extensions {
	fn equals<Other: ?Sized>(&self, other: &Other) -> bool
	where
		Self: PartialEq<Other>;
}

impl<T> Extensions for T {
	fn equals<Other: ?Sized>(&self, other: &Other) -> bool
	where
		Self: PartialEq<Other>,
	{
		self == other
	}
}
