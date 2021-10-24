use std::cmp::Ordering;

pub struct Comparator<T> {
	pub compare: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T> Comparator<T> {
	pub fn comparing<Extracted>(key_extractor: impl for<'a> Fn(&'a T) -> &'a Extracted + 'static) -> Self
	where
		Extracted: Ord,
	{
		Self {
			compare: Box::new(move |left, right| key_extractor(left).cmp(key_extractor(right))),
		}
	}

	pub fn then_comparing<KeyExtractor, Extracted>(self, key_extractor: KeyExtractor) -> Self
	where
		T: 'static,
		for<'a> KeyExtractor: 'static + Fn(&'a T) -> Extracted,
		for<'a> Extracted: Ord + 'a,
	{
		Self {
			compare: Box::new(move |left, right| match (self.compare)(left, right) {
				Ordering::Equal => key_extractor(left).cmp(&key_extractor(right)),
				other => other,
			}),
		}
	}

	pub fn reversed(self) -> Self
	where
		T: 'static,
	{
		use Ordering::*;
		Self {
			compare: Box::new(move |left, right| match (self.compare)(left, right) {
				Greater => Less,
				Equal => Equal,
				Less => Greater,
			}),
		}
	}
}
