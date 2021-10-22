#[macro_export]
macro_rules! r#static {
	($visibility:vis $name:ident : $item_type:ty = $expression:expr) => {
		#[allow(non_snake_case)]
		$visibility fn $name() -> &'static $item_type {
			::lazy_static::lazy_static! {
				static ref $name: $item_type = $expression;
			};
			&$name
		}
	};
}
